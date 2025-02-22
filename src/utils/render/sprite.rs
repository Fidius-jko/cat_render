use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec2, Vec3};
use wgpu::{BlendState, ShaderStages};

use crate::{
    context::AppContext,
    render::{
        bind_group::BindGroup,
        camera::CameraProjection,
        mesh::{Material, MaterialLayout, MaterialLayoutBuilder, Mesh, MeshRef},
        render_pipeline::PipelineOptions,
        small::{Rect, Transform},
        texture::Texture,
        Render,
    },
};
pub fn init_sprites(_context: &mut AppContext) {}
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
        layout: &SpriteLayout,
        width: f32,
        height: f32,
        transform: Transform,
        texture: Texture,
        rect: Option<Rect>,
    ) -> Self {
        let rect = match rect {
            Some(r) => r,
            None => Rect {
                min: Vec2::splat(0.),
                max: Vec2::new(texture.get_size().x as f32, texture.get_size().y as f32),
            },
        };
        let size = Vec2::new(width, height);
        let render = SpriteRender::new(size, transform, texture, layout, rect);
        Self {
            size,
            transform,
            render,
        }
    }
    pub fn set_rect(&mut self, rect: Rect) {
        self.render.update_rect(rect);
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
            self.render.render(render);
        }
    }
}

pub struct SpriteRender {
    material: Material,
    mesh: MeshRef,
    size: Vec2,
    transform: Transform,
    texture: Texture,
    updated_uniform: bool,
    updated_rect: bool,
    rect: Rect,
}
impl SpriteRender {
    pub fn render(&mut self, render: &mut Render) {
        if self.updated_uniform {
            self.updated_uniform = false;
            self.update_uniform();
        }
        if self.updated_rect {
            self.updated_rect = false;
            self.update_rect_inner();
        }
        render.use_camera_uniform_at(1);
        self.mesh.draw_with_material(render, &self.material);
    }
    pub fn new(
        size: Vec2,
        transform: Transform,
        texture: Texture,
        layout: &SpriteLayout,
        rect: Rect,
    ) -> Self {
        let uniform = SpriteUniform {
            view_proj: (transform.get_matrix() * Mat4::from_scale(Vec3::new(size.x, size.y, 1.)))
                .to_cols_array_2d(),
        };
        let rect2 = TextureRectUniform {
            size: [rect.min.x, rect.min.y, rect.max.x, rect.max.y],
            texture_size: [texture.get_size().x as f32, texture.get_size().y as f32],
        };
        let material = Material::from_layout(
            &layout.material_layout,
            vec![
                (0, bytemuck::bytes_of(&uniform).to_vec()),
                (3, bytemuck::bytes_of(&rect2).to_vec()),
            ],
            vec![(1, 2, texture.clone())],
        );

        Self {
            material,
            size,
            transform,
            mesh: layout.mesh.clone(),
            texture,
            updated_uniform: false,
            rect,
            updated_rect: false,
        }
    }
    pub fn update_rect(&mut self, rect: Rect) {
        self.rect = rect;
        self.updated_rect = true;
    }
    fn update_rect_inner(&mut self) {
        let rect = TextureRectUniform {
            size: [
                self.rect.min.x,
                self.rect.min.y,
                self.rect.max.x,
                self.rect.max.y,
            ],
            texture_size: [
                self.texture.get_size().x as f32,
                self.texture.get_size().y as f32,
            ],
        };
        self.material
            .update_uniform(3, bytemuck::bytes_of(&rect).to_vec());
    }
    pub fn update_size(&mut self, size: Vec2) {
        self.size = size;
        self.updated_uniform = true;
    }
    fn update_uniform(&mut self) {
        let uniform = SpriteUniform {
            view_proj: (self.transform.get_matrix()
                * Mat4::from_scale(Vec3::new(self.size.x, self.size.y, 1.)))
            .to_cols_array_2d(),
        };
        self.material
            .update_uniform(0, bytemuck::bytes_of(&uniform).to_vec());
    }
    pub fn update_transform(&mut self, transform: Transform) {
        self.transform = transform;
        self.updated_uniform = true;
    }
}

#[derive(Clone)]
pub struct SpriteLayout {
    material_layout: MaterialLayout,
    mesh: MeshRef,
}

impl SpriteLayout {
    pub fn new(context: &mut AppContext, camera: BindGroup) -> Self {
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
        material_layout.register_uniform_at(3, ShaderStages::VERTEX);
        let material_layout = material_layout.build(context.get_mut_renderer());
        Self {
            material_layout,
            mesh: Mesh::new(
                vec![
                    Vertex {
                        position: [0., 0., 0.],
                        tex_coords: [0., 0.],
                    },
                    Vertex {
                        position: [0., 1., 0.],
                        tex_coords: [0., 1.],
                    },
                    Vertex {
                        position: [1., 0., 0.],
                        tex_coords: [1., 0.],
                    },
                    Vertex {
                        position: [1., 1., 0.],
                        tex_coords: [1., 1.],
                    },
                ],
                vec![0, 3, 1, 0, 2, 3],
            )
            .ref_me(),
        }
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

#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct TextureRectUniform {
    size: [f32; 4],
    texture_size: [f32; 2],
}
