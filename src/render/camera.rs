use glam::{Mat4, UVec2, Vec2};
use wgpu::{BufferUsages, ShaderStages};

use super::{
    bind_group::{BindGroup, BindGroupEntryLayout, BindGroupEntryResources},
    buffer::Buffer,
    small::{Rect, Transform},
    surface::SurfaceId,
    Renderer,
};

pub struct Camera2DOptions {
    pub transform: Transform,
    pub near: f32,
    pub far: f32,
    pub viewport_origin: Vec2,
    pub scale: f32,
    pub surface: SurfaceId,
}
impl Default for Camera2DOptions {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            near: -100.,
            far: 1000.,
            viewport_origin: Vec2::splat(0.5),
            scale: 1.,
            surface: SurfaceId::default(),
        }
    }
}

pub struct Camera2D {
    transform: Transform,
    near: f32,
    far: f32,
    viewport_origin: Vec2,
    scale: f32,
    surface: SurfaceId,
    render: Option<CameraRender>,
    is_need_update: bool,
    area: Rect,
    window_size: UVec2,
}
impl Camera2D {
    pub fn new(opt: Camera2DOptions) -> Self {
        Self {
            transform: opt.transform,
            near: opt.near,
            far: opt.far,
            viewport_origin: opt.viewport_origin,
            scale: opt.scale,
            surface: opt.surface,
            render: None,
            area: Rect {
                min: Vec2::default(),
                max: Vec2::default(),
            },
            window_size: UVec2::default(),
            is_need_update: false,
        }
    }
    pub fn set_near(&mut self, near: f32) {
        self.near = near;
        self.is_need_update = true;
    }
    pub fn set_far(&mut self, far: f32) {
        self.far = far;
        self.is_need_update = true;
    }
    pub fn set_viewport_origin(&mut self, viewport_origin: Vec2) {
        self.viewport_origin = viewport_origin;
        self.is_need_update = true;
    }
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
        self.is_need_update = true;
    }

    pub fn get_near(&self) -> f32 {
        self.near
    }
    pub fn get_far(&self) -> f32 {
        self.far
    }
    pub fn get_viewport_origin(&self) -> Vec2 {
        self.viewport_origin
    }
    pub fn get_scale(&self) -> f32 {
        self.scale
    }
    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
        self.is_need_update = true;
    }
    pub fn get_transform(&self) -> Transform {
        self.transform.clone()
    }
    pub fn generate_matrix(&self) -> Mat4 {
        let mut transf = self.transform;
        transf.translation *= -1.;
        Mat4::orthographic_rh(
            self.area.min.x,
            self.area.max.x,
            self.area.min.y,
            self.area.max.y,
            self.far,
            self.near,
        ) * transf.get_matrix()
    }
    pub fn update_window_size(&mut self, width: u32, height: u32) {
        if self.window_size.x != width || self.window_size.y != height || self.is_need_update {
            self.is_need_update = false;
            self.window_size.x = width;
            self.window_size.y = height;
            let (projection_width, projection_height) = (width as f32, height as f32);

            let origin_x = projection_width * self.viewport_origin.x;
            let origin_y = projection_height * self.viewport_origin.y;

            self.area = Rect::new(
                self.scale * -origin_x,
                self.scale * -origin_y,
                self.scale * (projection_width - origin_x),
                self.scale * (projection_height - origin_y),
            );
            self.is_need_update = true;
        }
    }
}
impl Camera for Camera2D {
    fn get_render(&mut self, renderer: &mut Renderer, surface_size: (u32, u32)) -> &CameraRender {
        self.update_window_size(surface_size.0, surface_size.1);
        let uniform = CameraUniform {
            proj: self.generate_matrix().to_cols_array_2d(),
        };
        if let Some(renderr) = self.render.as_mut() {
            if self.is_need_update {
                renderr.buffer.update(&renderer, vec![uniform]);
            }
        } else {
            self.render = Some(CameraRender::new(&renderer, uniform));
        }
        self.render.as_ref().unwrap()
    }
    fn get_surface_id(&self) -> SurfaceId {
        self.surface.clone()
    }
}

pub trait Camera {
    fn get_surface_id(&self) -> SurfaceId;
    fn get_render(&mut self, renderer: &mut Renderer, surface_size: (u32, u32)) -> &CameraRender;
}

#[derive(Clone)]
pub struct CameraRender {
    pub buffer: Buffer<CameraUniform>,
    pub bindgroup: BindGroup,
}
impl CameraRender {
    pub fn new(renderer: &Renderer, uniform: CameraUniform) -> Self {
        let buf = renderer.create_buffer(
            vec![uniform],
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        );
        Self {
            bindgroup: BindGroup::new(
                renderer,
                vec![BindGroupEntryLayout {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                }],
                vec![BindGroupEntryResources {
                    binding: 0,
                    resource: buf.as_entire_binding(),
                }],
            ),
            buffer: buf,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    proj: [[f32; 4]; 4],
}
