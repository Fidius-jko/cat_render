use cat_render::{
    prelude::*,
    render::{
        render_pipeline::{
            BlendState, ColorWrites, MultisampleState, PipelineId, PipelineOptions, PrimitiveState,
        },
        surface::SurfaceId,
        Color,
    },
    utils::fs::Filesystem,
};

fn main() {
    let _ = App::run();
}

pub struct App {
    surface: SurfaceId,
    pipeline: PipelineId,
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

        let pipeline = context.get_mut_renderer().create_pipeline(PipelineOptions {
            vertex_shader: Filesystem::get()
                .read_to_string("assets/shader.wgsl")
                .unwrap(),
            vertex_entry_point: String::from("vs_main"),
            fragment_shader: None,
            fragment_entry_point: String::from("fs_main"),
            frag_blend: Some(BlendState::REPLACE),
            write_mask: ColorWrites::all(),
            bind_group_layouts: vec![],
            buffers: vec![],
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
        Self { surface, pipeline }
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
                render.set_pipeline(self.pipeline.clone());
                render.draw(0..3, 0..1);
            },
        );
    }
}
