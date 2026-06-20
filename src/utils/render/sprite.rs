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
    box_rect: Rect,
    is_have_rect: bool,
    origin: Vec2,
}

impl Sprite {
    pub fn update_texture(&mut self, texture: Texture) {
        self.render.change_texture(texture.clone());
        if self.size.x == 0. || self.size.y == 0. {
            let mut width = self.size.x;
            let mut height = self.size.y;
            if width == 0. {
                width = texture.get_size().x as f32;
            }
            if height == 0. {
                height = texture.get_size().y as f32;
            }
            self.render.update_size(Vec2::new(width, height));
            self.update_box();
        }
        if !self.is_have_rect {
            let rect = Rect {
                min: Vec2::splat(0.),
                max: Vec2::new(texture.get_size().x as f32, texture.get_size().y as f32),
            };

            self.render.update_rect(rect);
        }
    }
    pub fn get_transform(&self) -> Transform {
        self.transform
    }
    pub fn get_texture(&self) -> Texture {
        self.render.texture.clone()
    }
    pub fn new(
        layout: &SpriteLayout,
        orig_width: f32,
        orig_height: f32,
        transform: Transform,
        texture: Texture,
        orig_rect: Option<Rect>,
    ) -> Self {
        let mut width = orig_width;
        let mut height = orig_height;
        if width == 0. {
            width = texture.get_size().x as f32;
        }
        if height == 0. {
            height = texture.get_size().y as f32;
        }
        let rect = match orig_rect {
            Some(r) => r,
            None => Rect {
                min: Vec2::splat(0.),
                max: Vec2::new(texture.get_size().x as f32, texture.get_size().y as f32),
            },
        };
        let size = Vec2::new(orig_width, orig_height);
        let render = SpriteRender::new(Vec2::new(width, height), transform, texture, layout, rect);
        let mut s = Self {
            size,
            transform,
            render,
            box_rect: Rect::new(0., 0., size.x, size.y).transformed(transform),
            is_have_rect: orig_rect.is_some(),
            origin: layout.origin,
        };
        s.update_box();
        s
    }
    pub fn set_rect(&mut self, rect: Rect) {
        self.is_have_rect = true;
        self.render.update_rect(rect);
    }
    fn update_box(&mut self) {
        let mut width = self.size.x;
        let mut height = self.size.y;
        if width == 0. {
            width = self.render.texture.get_size().x as f32;
        }
        if height == 0. {
            height = self.render.texture.get_size().y as f32;
        }
        let rect = Rect::new(
            0. - self.origin.x,
            0. + self.origin.y,
            1. - self.origin.x,
            1. + self.origin.y,
        );
        let rect = rect * Vec2::new(width, height);
        self.box_rect = rect.transformed(self.transform);
    }
    pub fn update_size(&mut self, mut new_size: Vec2) {
        self.size = new_size;
        if new_size.x == 0. {
            new_size.x = self.render.texture.get_size().x as f32;
        }
        if new_size.y == 0. {
            new_size.y = self.render.texture.get_size().y as f32;
        }
        self.render.update_size(new_size);
        self.update_box();
    }
    pub fn update_transform(&mut self, transform: Transform) {
        self.transform = transform;
        self.render.update_transform(transform);
        self.update_box();
    }
    pub fn render(&mut self, render: &mut Render) {
        let proj = render.get_projection();
        let need_render = match proj {
            CameraProjection::P2D {
                near: _,
                far: _,
                area,
            } => {
                area.is_inserction(self.box_rect)
                // && self.transform.translation.z >= near
                // && self.transform.translation.z <= far
            }
        };
        if need_render {
            self.render.render(render);
        }
    }
}

pub struct SpriteRender {
    mesh: MeshRef,
    size: Vec2,
    transform: Transform,
    texture: Texture,
    updated_uniform: bool,
    updated_rect: bool,
    updated_texture: bool,
    rect: Rect,

    material: Option<Material>,
    layout: Option<MaterialLayout>,
}
impl SpriteRender {
    pub fn render(&mut self, render: &mut Render) {
        if self.material.is_none() {
            let rect = self.get_texture_rect();
            let uni = self.get_uniform();
            self.material = Some(Material::from_layout(
                self.layout.as_ref().unwrap(),
                vec![
                    (0, bytemuck::bytes_of(&uni).to_vec()),
                    (3, bytemuck::bytes_of(&rect).to_vec()),
                ],
                vec![(1, 2, self.texture.clone())],
            ));
            self.updated_uniform = true;
            self.updated_rect = true;
            self.updated_texture = false;
        }

        if self.updated_uniform {
            self.updated_uniform = false;
            self.update_uniform();
        }
        if self.updated_rect {
            self.updated_rect = false;
            self.update_rect_inner();
        }
        if self.updated_texture {
            self.updated_texture = false;
            self.material
                .as_mut()
                .unwrap()
                .change_textures(vec![(1, 2, self.texture.clone())]);
        }
        render.use_camera_uniform_at(1);
        self.mesh
            .draw_with_material(render, self.material.as_mut().unwrap());
    }
    pub fn change_texture(&mut self, texture: Texture) {
        self.texture = texture;
        self.updated_texture = true;
    }
    pub fn new(
        size: Vec2,
        transform: Transform,
        texture: Texture,
        layout: &SpriteLayout,
        rect: Rect,
    ) -> Self {
        Self {
            material: None,
            layout: Some(layout.material_layout.clone()),
            size,
            transform,
            mesh: layout.mesh.clone(),
            texture,
            updated_uniform: false,
            rect,
            updated_rect: false,
            updated_texture: false,
        }
    }
    pub fn update_rect(&mut self, rect: Rect) {
        self.rect = rect;
        self.updated_rect = true;
    }
    fn get_texture_rect(&self) -> TextureRectUniform {
        TextureRectUniform {
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
        }
    }
    fn update_rect_inner(&mut self) {
        let rect = self.get_texture_rect();
        self.material
            .as_mut()
            .unwrap()
            .update_uniform(3, bytemuck::bytes_of(&rect).to_vec());
    }
    pub fn update_size(&mut self, size: Vec2) {
        self.size = size;
        self.updated_uniform = true;
    }
    fn get_uniform(&self) -> SpriteUniform {
        SpriteUniform {
            view_proj: (self.transform.get_matrix()
                * Mat4::from_scale(Vec3::new(self.size.x, self.size.y, 1.)))
            .to_cols_array_2d(),
        }
    }
    fn update_uniform(&mut self) {
        let uni = self.get_uniform();
        self.material
            .as_mut()
            .unwrap()
            .update_uniform(0, bytemuck::bytes_of(&uni).to_vec());
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
    origin: Vec2,
}

impl SpriteLayout {
    pub fn new(context: &mut AppContext, camera: BindGroup, origin: Option<Vec2>) -> Self {
        let origin = if let Some(o) = origin {
            o
        } else {
            Vec2::splat(0.5)
        };
        let mut material_layout = MaterialLayoutBuilder::new(PipelineOptions {
            vertex_shader: include_str!("sprite_shader.wgsl").to_string(),
            vertex_entry_point: String::from("vs_main"),
            fragment_entry_point: String::from("fs_main"),
            bind_group_layouts: vec![camera.layout()],
            buffers: vec![Vertex::desc()],
            frag_blend: Some(BlendState::ALPHA_BLENDING),
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
                        position: [0. - origin.x, 0. + origin.y, 0.],
                        tex_coords: [0., 0.],
                    },
                    Vertex {
                        position: [0. - origin.x, -1. + origin.y, 0.],
                        // position: [0., 1., 0.],
                        tex_coords: [0., 1.],
                    },
                    Vertex {
                        position: [1. - origin.x, 0. + origin.y, 0.],
                        tex_coords: [1., 0.],
                    },
                    Vertex {
                        position: [1. - origin.x, -1. + origin.y, 0.],
                        // position: [1., 1., 0.],
                        tex_coords: [1., 1.],
                    },
                ],
                vec![3, 2, 0, 0, 1, 3],
                // vec![0, 3, 1, 0, 2, 3],
            )
            .ref_me(),
            origin,
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
