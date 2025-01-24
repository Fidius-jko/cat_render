use cat_render::{
    prelude::*,
    render::{
        bind_group::{BindGroup, BindGroupEntry, BindingResource, BindingType, ShaderStages},
        buffer::{Buffer, BufferUsages},
        bytemuck::{Pod, Zeroable},
        render_pipeline::{
            BlendState, ColorWrites, MultisampleState, PipelineId, PipelineOptions, PrimitiveState,
        },
        surface::SurfaceId,
        Color,
    },
    utils::fs::Filesystem,
};
use glam::Vec3;
use winit::{
    event::KeyEvent,
    keyboard::{KeyCode, PhysicalKey},
};

fn main() {
    let _ = App::run();
}

pub struct App {
    surface: SurfaceId,
    pipeline: PipelineId,
    buffer: Buffer<Vertex>,
    index_buffer: Buffer<u16>,
    texture_bind_group: BindGroup,
    uniform_buf: Buffer<MyUniform>,
    uniform_bind_group: BindGroup,
}

impl CatApp for App {
    fn config() -> AppConfig {
        AppConfig {
            loop_type: LoopType::Active,
        }
    }
    fn new(context: &mut AppContext) -> Self {
        let window = context.create_window(WindowAttributes::default().with_title("Test example"));
        let surface = context.create_surface_for_window(&window).unwrap();

        let texture = context
            .get_mut_renderer()
            .create_texture_from_bytes(&Filesystem::get().read("assets/happy-tree.png").unwrap())
            .unwrap();
        let texture_bind_group = context.get_mut_renderer().create_bind_group(vec![
            BindGroupEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                resource: BindingResource::TextureView(&texture.view),
            },
            BindGroupEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                resource: BindingResource::Sampler(&texture.sampler),
            },
        ]);
        let view_proj = glam::Mat4::from_scale(Vec3::new(0.5, 2.0, 0.))
            * glam::Mat4::from_translation(Vec3::new(0.0, -0.0, 0.));
        let uniform_buf = context.get_mut_renderer().create_buffer(
            vec![MyUniform {
                view_proj: view_proj.to_cols_array_2d(),
            }],
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        );
        let uniform_bind_group =
            context
                .get_mut_renderer()
                .create_bind_group(vec![BindGroupEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    resource: uniform_buf.as_entire_binding(),
                }]);
        let pipeline = context.get_mut_renderer().create_pipeline(PipelineOptions {
            vertex_shader: Filesystem::get()
                .read_to_string("assets/shader.wgsl")
                .unwrap(),
            vertex_entry_point: String::from("vs_main"),
            fragment_shader: None,
            fragment_entry_point: String::from("fs_main"),
            frag_blend: Some(BlendState::REPLACE),
            write_mask: ColorWrites::all(),
            bind_group_layouts: vec![texture_bind_group.layout(), uniform_bind_group.layout()],
            buffers: vec![Vertex::desc()],
            primitive: PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            cache: None,
        });
        #[rustfmt::skip]
        let vertices = vec![
            Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.99240386], }, // A
            Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.56958647], }, // B
            Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.05060294], }, // C
            Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.1526709], }, // D
            Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.7347359], }, // E
        ];
        let indices: Vec<u16> = vec![0, 1, 4, 1, 2, 4, 2, 3, 4];
        let buffer = context
            .get_mut_renderer()
            .create_buffer(vertices, BufferUsages::VERTEX);
        let index_buffer = context
            .get_mut_renderer()
            .create_buffer(indices, BufferUsages::INDEX);

        Self {
            surface,
            pipeline,
            buffer,
            index_buffer,
            texture_bind_group,
            uniform_bind_group,
            uniform_buf,
        }
    }
    fn update(&mut self, _context: &mut AppContext) {}
    fn window_event(&mut self, event: WindowEvent, context: &mut AppContext, _window: CatWindow) {
        match event {
            WindowEvent::CloseRequested => {
                context.exit();
            }
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => {
                let KeyEvent { physical_key, .. } = event;

                if physical_key == PhysicalKey::Code(KeyCode::KeyA) {
                    let view_proj = glam::Mat4::from_scale(Vec3::new(0.5, 2.0, 0.))
                        * glam::Mat4::from_translation(Vec3::new(0.5, -0.25, 0.));
                    context.get_renderer().update_buffer(
                        vec![MyUniform {
                            view_proj: view_proj.to_cols_array_2d(),
                        }],
                        &self.uniform_buf,
                    );
                }
            }
            _ => {}
        }
    }
    fn render(&mut self, render: &mut cat_render::render::Renderer) {
        render.start_render_for_surface(
            self.surface.clone(),
            Some(Color::srgb_255(200., 200., 200.)),
            |render| {
                render.set_pipeline(self.pipeline.clone());
                render.set_bind_group(1, &self.uniform_bind_group, &[]);
                render.set_bind_group(0, &self.texture_bind_group, &[]);
                render.set_vertex_buffer(&self.buffer, 0, ..);
                render.set_index_buffer(&self.index_buffer, ..);
                render.draw_indexed(0..self.index_buffer.get_vertices_number(), 0, 0..1);
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
