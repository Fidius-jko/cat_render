use bytemuck::{Pod, Zeroable};
use cat_render::{
    prelude::*,
    render::{
        camera::{Camera2D, Camera2DOptions},
        mesh::{Material, MaterialLayoutBuilder, Mesh},
        render_pipeline::PipelineOptions,
        small::Transform,
    },
    utils::fs::Filesystem,
};
use wgpu::ShaderStages;
use winit::{
    event::{KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

fn main() {
    let _ = App::run();
}

pub struct App {
    material: Material,
    mesh: Mesh<Vertex>,
    camera: Camera2D,
}

impl CatApp for App {
    fn config() -> AppConfig {
        AppConfig {
            loop_type: LoopType::Active,
        }
    }
    fn new(context: &mut AppContext) -> Self {
        let window =
            context.create_window(WindowAttributes::default().with_title("Objects example"));
        let surface = context.create_surface_for_window(&window).unwrap();
        // let surface_size = context.get_renderer().get_surface_size(surface.clone());
        let camera = Camera2D::new(
            context.get_renderer(),
            Camera2DOptions {
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                surface: surface.clone(),
                viewport_origin: Vec2::new(0.5, 0.5),
                ..Default::default()
            },
        );
        let texture = context
            .get_mut_renderer()
            .create_texture_from_bytes(&Filesystem::get().read("assets/happy-tree.png").unwrap())
            .unwrap();

        let mut material_layout = MaterialLayoutBuilder::new(PipelineOptions {
            vertex_shader: Filesystem::get()
                .read_to_string("assets/shader.wgsl")
                .unwrap(),
            vertex_entry_point: String::from("vs_main"),
            fragment_entry_point: String::from("fs_main"),
            buffers: vec![Vertex::desc()],
            bind_group_layouts: vec![camera.get_bind_group().layout()],
            ..Default::default()
        });
        material_layout.register_uniform_at(0, ShaderStages::VERTEX_FRAGMENT);
        material_layout.register_texture_at(1, 2, ShaderStages::VERTEX_FRAGMENT);
        let material_layout = material_layout.build(context.get_mut_renderer());

        let view_proj = glam::Mat4::from_scale(Vec3::new(2., 2.0, 0.))
            * glam::Mat4::from_translation(Vec3::new(0.0, -5.0, 0.));

        let material = Material::from_layout(
            context.get_renderer(),
            &material_layout,
            vec![(
                0,
                bytemuck::bytes_of(&MyUniform {
                    view_proj: view_proj.to_cols_array_2d(),
                })
                .to_vec(),
            )],
            vec![(1, 2, texture.clone())],
        );
        #[rustfmt::skip]
        let vertices = vec![
            Vertex { position: [-8.68241, 49.240386, 0.0] , tex_coords: [0.4131759, 0.99240386], }, // A
            Vertex { position: [-49.513406, 6.958647, 0.0] , tex_coords: [0.0048659444, 0.56958647], }, // B
            Vertex { position: [-21.918549, -44.939706, 0.0], tex_coords: [0.28081453, 0.05060294], }, // C
            Vertex { position: [35.966998, -34.73291, 0.0], tex_coords: [0.85967, 0.1526709], }, // D
            Vertex { position: [44.147372, 23.47359, 0.0], tex_coords: [0.9414737, 0.7347359], }, // E
        ];
        let indices: Vec<u16> = vec![0, 1, 4, 1, 2, 4, 2, 3, 4];
        let mesh = Mesh::new(vertices, indices);
        Self {
            material,
            mesh,
            camera,
        }
    }
    fn update(&mut self, _context: &mut AppContext) {}
    fn window_event(&mut self, event: WindowEvent, context: &mut AppContext, _window: CatWindow) {
        match event {
            WindowEvent::CloseRequested => {
                context.exit();
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                let KeyEvent {
                    physical_key,
                    logical_key: _,
                    text: _,
                    location: _,
                    state: _,
                    repeat,
                    ..
                } = event;
                if physical_key == PhysicalKey::Code(KeyCode::KeyO) && !repeat {
                    self.camera.set_scale(self.camera.get_scale() - 0.1);
                }
                if physical_key == PhysicalKey::Code(KeyCode::KeyI) && !repeat {
                    self.camera.set_scale(self.camera.get_scale() + 0.1);
                }
                let mut trans = Vec3::splat(0.);
                if physical_key == PhysicalKey::Code(KeyCode::KeyA) {
                    trans.x -= 1.;
                }
                if physical_key == PhysicalKey::Code(KeyCode::KeyD) {
                    trans.x += 1.;
                }
                if physical_key == PhysicalKey::Code(KeyCode::KeyW) {
                    trans.y += 1.;
                }
                if physical_key == PhysicalKey::Code(KeyCode::KeyS) {
                    trans.y -= 1.;
                }
                let mut transform = self.camera.get_transform();
                const SPEED: f32 = 10.;
                transform.translation += trans * SPEED;
                self.camera.set_transform(transform);
            }
            _ => {}
        }
    }
    fn render(&mut self, render: &mut cat_render::render::Renderer) {
        render.start_render_for_camera(
            &mut self.camera,
            Some(Color::srgb_255(200., 200., 200.)),
            |render| {
                render.use_camera_uniform_at(1);
                self.mesh.draw_with_material(render, &self.material);
            },
        );
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
struct MyUniform {
    view_proj: [[f32; 4]; 4],
}
