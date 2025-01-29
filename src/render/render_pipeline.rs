use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

pub use wgpu::{PipelineCompilationOptions, PipelineLayoutDescriptor};
use wgpu::{PipelineLayout, ShaderModule, TextureFormat};

pub use wgpu::RenderPipeline;

use super::Renderer;

pub(crate) struct Pipelines {
    pipelines: HashMap<PipelineId, Pipeline>,
    last_id: u32,
}

pub struct Pipeline {
    vert_shader: ShaderModule,
    frag_shader: ShaderModule,
    render_pipeline_layout: PipelineLayout,
    options: PipelineOptions,
    builded: HashMap<TextureFormat, Rc<RenderPipeline>>,
}
impl Pipelines {
    pub fn new() -> Self {
        Self {
            pipelines: HashMap::new(),
            last_id: 0,
        }
    }
    pub fn get_pipeline_for_surface(
        &mut self,
        format: TextureFormat,
        pipeline_id: PipelineId,
        renderer: &Renderer,
    ) -> Rc<RenderPipeline> {
        let pipeline = self
            .pipelines
            .get_mut(&pipeline_id)
            .expect("Pipeline doesn't exist");
        match pipeline.builded.get(&format) {
            Some(_) => {}
            None => {
                let render_pipeline =
                    renderer
                        .device
                        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                            label: Some("Render Pipeline"),
                            layout: Some(&pipeline.render_pipeline_layout),
                            vertex: wgpu::VertexState {
                                module: &pipeline.vert_shader,
                                entry_point: Some(&pipeline.options.vertex_entry_point),
                                buffers: pipeline.options.buffers.as_slice(),
                                compilation_options: PipelineCompilationOptions::default(),
                            },
                            fragment: Some(wgpu::FragmentState {
                                module: &pipeline.frag_shader,
                                entry_point: Some(&pipeline.options.fragment_entry_point),
                                targets: &[Some(wgpu::ColorTargetState {
                                    format,
                                    blend: pipeline.options.frag_blend,
                                    write_mask: pipeline.options.write_mask,
                                })],
                                compilation_options: PipelineCompilationOptions::default(),
                            }),
                            primitive: pipeline.options.primitive,
                            depth_stencil: pipeline.options.depth_stencil.clone(),
                            multisample: pipeline.options.multisample,
                            multiview: None,
                            cache: None,
                        });
                pipeline
                    .builded
                    .insert(format.clone(), Rc::new(render_pipeline));
            }
        };
        pipeline.builded.get(&format).unwrap().clone()
    }
    pub fn create_pipeline(&mut self, renderer: &Renderer, options: PipelineOptions) -> PipelineId {
        let vert_shader = renderer
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Vertex Shader"),
                source: wgpu::ShaderSource::Wgsl(options.vertex_shader.clone().into()),
            });
        let frag_shader = match options.fragment_shader {
            Some(ref f) => renderer
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Vertex Shader"),
                    source: wgpu::ShaderSource::Wgsl(f.into()),
                }),
            None => renderer
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Vertex Shader"),
                    source: wgpu::ShaderSource::Wgsl(options.vertex_shader.clone().into()),
                }),
        };
        let render_pipeline_layout =
            renderer
                .device
                .create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: Some("Pipeline layout"),
                    bind_group_layouts: options
                        .bind_group_layouts
                        .iter()
                        .map(|a| a.borrow())
                        .collect::<Vec<&BindGroupLayout>>()
                        .as_slice(),
                    push_constant_ranges: &[], // Needs feature
                });
        self.pipelines.insert(
            PipelineId(self.last_id),
            Pipeline {
                vert_shader,
                frag_shader,
                render_pipeline_layout,
                options,
                builded: HashMap::new(),
            },
        );
        self.last_id += 1;
        return PipelineId(self.last_id - 1);
    }
}

pub struct PipelineOptions {
    pub vertex_shader: String,
    pub vertex_entry_point: String,
    pub fragment_shader: Option<String>,
    pub fragment_entry_point: String,
    pub frag_blend: Option<BlendState>,
    pub write_mask: ColorWrites,
    pub bind_group_layouts: Vec<Arc<BindGroupLayout>>,
    pub buffers: Vec<VertexBufferLayout<'static>>,
    pub primitive: wgpu::PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
    /// The multi-sampling properties of the pipeline.
    pub multisample: MultisampleState,
    /// The pipeline cache to use when creating this pipeline.
    pub cache: Option<PipelineCache>,
}
impl Default for PipelineOptions {
    fn default() -> Self {
        PipelineOptions {
            vertex_shader: "YOUR SHADER".to_string(),
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
        }
    }
}
pub type BindGroupLayout = wgpu::BindGroupLayout;
pub type BlendState = wgpu::BlendState;
pub type ColorWrites = wgpu::ColorWrites;
pub type DepthStencilState = wgpu::DepthStencilState;
pub type MultisampleState = wgpu::MultisampleState;
pub type PipelineCache = wgpu::PipelineCache;
pub type PrimitiveState = wgpu::PrimitiveState;
pub type VertexBufferLayout<'a> = wgpu::VertexBufferLayout<'a>;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct PipelineId(u32);
