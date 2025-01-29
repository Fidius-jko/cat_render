use bytemuck::{Pod, Zeroable};
use cat_render::{
    prelude::*,
    render::{
        mesh::{Material, MaterialLayoutBuilder, Mesh},
        render_pipeline::PipelineOptions,
    },
    utils::fs::Filesystem,
};
use wgpu::ShaderStages;

fn main() {
    let _ = App::run();
}

pub struct App {
    surface: SurfaceId,
    material: Material,
    mesh: Mesh<Vertex>,
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
            ..Default::default()
        });
        material_layout.register_uniform_at(0, ShaderStages::VERTEX_FRAGMENT);
        material_layout.register_texture_at(1, 2, ShaderStages::VERTEX_FRAGMENT);
        let material_layout = material_layout.build(context.get_mut_renderer());

        let view_proj = glam::Mat4::from_scale(Vec3::new(0.5, 2.0, 0.))
            * glam::Mat4::from_translation(Vec3::new(0.0, -0.0, 0.));

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
            Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.99240386], }, // A
            Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.56958647], }, // B
            Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.05060294], }, // C
            Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.1526709], }, // D
            Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.7347359], }, // E
        ];
        let indices: Vec<u16> = vec![0, 1, 4, 1, 2, 4, 2, 3, 4];
        let mesh = Mesh::new(vertices, indices);
        Self {
            surface,
            material,
            mesh,
        }
    }
    fn update(&mut self, _context: &mut AppContext) {}
    fn window_event(&mut self, event: WindowEvent, context: &mut AppContext, _window: CatWindow) {
        match event {
            WindowEvent::CloseRequested => {
                context.exit();
            }
            _ => {}
        }
    }
    fn render(&mut self, render: &mut cat_render::render::Renderer) {
        render.start_render_for_surface(
            self.surface.clone(),
            Some(Color::srgb_255(200., 200., 200.)),
            |render| {
                self.mesh.draw_with_material(render, &self.material, 0);
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
