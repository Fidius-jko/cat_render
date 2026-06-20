use bytemuck::{Pod, Zeroable};
use cat_render::{
    prelude::*,
    render::{
        camera::{Camera, Camera2D, Camera2DOptions},
        mesh::{Material, MaterialLayoutBuilder, Mesh},
        render_pipeline::PipelineOptions,
        small::Transform,
    },
    utils::{fs::Filesystem, input::Input},
};
use wgpu::ShaderStages;
use winit::{event::WindowEvent, keyboard::KeyCode};

fn main() {
    let _ = App::run();
}

pub struct App {
    material: Material,
    mesh: Mesh<Vertex>,
    camera: Camera2D,
    input: Input,
}

impl CatApp for App {
    fn config() -> AppConfig {
        AppConfig {
            loop_type: LoopType::Active,
        }
    }
    fn new(context: &mut AppContext) -> Self {
        cat_render::utils::init_utils(context);
        let data = std::fs::read("assets/FiraMono-Medium.ttf").unwrap();
        let face = ttf_parser::Face::parse(&data, 0).unwrap();
        let id = face.glyph_index('A').unwrap();
        let rect = face.glyph_bounding_box(id).unwrap();

        let window =
            context.create_window(WindowAttributes::default().with_title("Objects example"));
        let surface = context.create_surface_for_window(&window).unwrap();
        let camera = Camera2D::new(Camera2DOptions {
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            surface: surface.clone(),
            viewport_origin: Vec2::new(0.5, 0.5),
            ..Default::default()
        });
        let texture = context
            .get_mut_renderer()
            .create_texture_from_bytes(
                &Filesystem::get().read("assets/happy-tree.png").unwrap(),
                wgpu::FilterMode::Nearest,
            )
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
        ];
        let indices: Vec<u16> = vec![0, 1, 4, 1, 2, 4, 2, 3, 4];
        let mesh = Mesh::new(vertices, indices);
        Self {
            material,
            mesh,
            camera,
            input: Input::new(),
        }
    }
    fn update(&mut self, _context: &mut AppContext, _delta: f32) {
        if self.input.is_pressed_key(KeyCode::KeyO) {
            self.camera.set_scale(self.camera.get_scale() - 0.1);
        }
        if self.input.is_pressed_key(KeyCode::KeyI) {
            self.camera.set_scale(self.camera.get_scale() + 0.1);
        }
        let mut trans = Vec3::splat(0.);
        if self.input.is_down_key(KeyCode::KeyA) {
            trans.x -= 1.;
        }
        if self.input.is_down_key(KeyCode::KeyD) {
            trans.x += 1.;
        }
        if self.input.is_down_key(KeyCode::KeyW) {
            trans.y += 1.;
        }
        if self.input.is_down_key(KeyCode::KeyS) {
            trans.y -= 1.;
        }
        let mut transform = self.camera.get_transform();
        const SPEED: f32 = 1.;
        transform.translation += trans * SPEED;
        self.camera.set_transform(transform);
        self.input.tick();
    }
    fn window_event(&mut self, event: WindowEvent, context: &mut AppContext, _window: CatWindow) {
        self.input.window_event(event.clone());
        match event {
            WindowEvent::CloseRequested => {
                context.exit();
            }
            _ => {}
        }
    }
    fn render(&mut self, render: &mut cat_render::render::Renderer) {
        render.start_render_for_surface(
            self.camera.get_surface_id(),
            Some(Color::srgb_255(200., 200., 200.)),
            None,
            |render| {
                render.set_camera(&mut self.camera);
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
