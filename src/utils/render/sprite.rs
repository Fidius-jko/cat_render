use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use wgpu::{BlendState, ShaderStages};

use crate::{
    context::{AppContext, Resources},
    render::{
        bind_group::BindGroup,
        camera::{Camera2D, CameraProjection},
        mesh::{Material, MaterialLayout, MaterialLayoutBuilder, Mesh},
        render_pipeline::PipelineOptions,
        small::{Rect, Transform},
        texture::Texture,
        Render, Renderer,
    },
};
pub fn init_sprites(context: &mut AppContext) {}
pub struct Sprite {
    size: Vec2,
    transform: Transform,
    render: SpriteRender,
}

impl Sprite {
    pub fn get_transform(&self) -> Transform {
        self.transform.clone()
    }
    pub fn new(
        context: &mut AppContext,
        camera: &mut Camera2D,
        width: f32,
        height: f32,
        transform: Transform,
        texture: Texture,
    ) -> Self {
        let size = Vec2::new(width, height);
        let render = SpriteRender::new(
            context.get_mut_renderer(),
            size,
            camera.get_bind_group(),
            transform,
            texture,
        );
        Self {
            size,
            transform,
            render,
        }
    }
    pub fn update_size(&mut self, new_size: Vec2) {
        self.size = new_size;
        self.render.update_size(new_size);
    }
    pub fn update_transform(&mut self, transform: Transform) {
        self.transform = transform;
        self.render.update_transform(transform);
    }
    pub fn render(&mut self, render: &mut Render) {
        let proj = render.get_projection();
        let rect = Rect::new(0., 0., self.size.x, self.size.y).transformed(self.transform);
        let need_render = match proj {
            CameraProjection::P2D { near, far, area } => {
                area.is_inserction(rect)
                    && self.transform.translation.z >= near
                    && self.transform.translation.z <= far
            }
        };
        if need_render {
            render.use_camera_uniform_at(1);
            self.render
                .mesh
                .draw_with_material(render, &self.render.material);
        }
    }
}

pub struct SpriteRender {
    material: Material,
    mesh: Mesh<Vertex>,
}
impl SpriteRender {
    pub fn new(
        renderer: &mut Renderer,
        size: Vec2,
        camera: BindGroup,
        transform: Transform,
        texture: Texture,
    ) -> Self {
        let layout = SpriteLayout::get_or_init(renderer, camera);

        let uniform = SpriteUniform {
            view_proj: transform.get_matrix().to_cols_array_2d(),
        };
        let material = Material::from_layout(
            &layout.material_layout,
            vec![(0, bytemuck::bytes_of(&uniform).to_vec())],
            vec![(1, 2, texture)],
        );
        let mesh = Mesh::new(
            vec![
                Vertex {
                    position: [0., 0., 0.],
                    tex_coords: [0., 0.],
                },
                Vertex {
                    position: [0., size.y, 0.],
                    tex_coords: [0., 1.],
                },
                Vertex {
                    position: [size.x, 0., 0.],
                    tex_coords: [1., 0.],
                },
                Vertex {
                    position: [size.x, size.y, 0.],
                    tex_coords: [1., 1.],
                },
            ],
            vec![0, 3, 1, 0, 2, 3],
        );
        Self { mesh, material }
    }
    pub fn update_size(&mut self, size: Vec2) {
        self.mesh.update(
            vec![
                Vertex {
                    position: [0., 0., 0.],
                    tex_coords: [0., 0.],
                },
                Vertex {
                    position: [0., size.y, 0.],
                    tex_coords: [0., 1.],
                },
                Vertex {
                    position: [size.x, 0., 0.],
                    tex_coords: [1., 0.],
                },
                Vertex {
                    position: [size.x, size.y, 0.],
                    tex_coords: [1., 1.],
                },
            ],
            vec![0, 3, 1, 0, 2, 3],
        );
    }
    pub fn update_transform(&mut self, transform: Transform) {
        let uniform = SpriteUniform {
            view_proj: transform.get_matrix().to_cols_array_2d(),
        };
        self.material
            .update_uniform(0, bytemuck::bytes_of(&uniform).to_vec());
    }
}

#[derive(Clone)]
pub struct SpriteLayout {
    material_layout: MaterialLayout,
}

impl SpriteLayout {
    pub fn get_or_init(renderer: &mut Renderer, camera: BindGroup) -> Self {
        let me = match Resources::get_me().get::<Self>() {
            Some(r) => return r.clone(),
            None => {
                let mut material_layout = MaterialLayoutBuilder::new(PipelineOptions {
                    vertex_shader: include_str!("sprite_shader.wgsl").to_string(),
                    vertex_entry_point: String::from("vs_main"),
                    fragment_entry_point: String::from("fs_main"),
                    bind_group_layouts: vec![camera.layout()],
                    buffers: vec![Vertex::desc()],
                    frag_blend: Some(BlendState::ALPHA_BLENDING),
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: Texture::DEPTH_FORMAT,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }),
                    ..Default::default()
                });
                material_layout.register_texture_at(1, 2, ShaderStages::FRAGMENT);
                material_layout.register_uniform_at(0, ShaderStages::VERTEX);
                let material_layout = material_layout.build(renderer);
                let me = Self { material_layout };
                me
            }
        };
        Resources::get_me_mut().insert(me.clone());
        me
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}
impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct SpriteUniform {
    view_proj: [[f32; 4]; 4],
}
