// use std::collections::HashMap;

// use bytemuck::{Pod, Zeroable};
// use glam::Vec3;
// use wgpu::BufferUsages;

// use crate::{
//     context::AppContext,
//     prelude::SurfaceId,
//     render::{
//         bind_group::{
//             BindGroup, BindGroupEntryLayout, BindGroupEntryResources, BindGroupLayout, BindingType,
//             ShaderStages,
//         },
//         buffer::Buffer,
//         render_pipeline::{
//             BlendState, ColorWrites, MultisampleState, PipelineId, PipelineOptions, PrimitiveState,
//         },
//         texture::Texture,
//         Renderer,
//     },
// };

// pub struct Sprite {
//     buffer: Buffer<Vertex>,
//     index_buffer: Buffer<u16>,
//     uniform_buf: Buffer<Uniform>,
//     texture: Texture,
//     window_res: HashMap<SurfaceId, Buffer<WindowResUniform>>,

//     pipeline: PipelineId,
//     bind_group: BindGroup,
// }

// impl Sprite {
//     pub fn new(context: &mut AppContext, texture: &[u8]) -> Self {
//         let resources = SpritesResources::get_or_init(context);
//         let layout = resources.bind_group_layout.clone();
//         let pipeline = resources.pipeline.clone();
//         let texture = context
//             .get_mut_renderer()
//             .create_texture_from_bytes(texture)
//             .unwrap();
//         let buffer = context.get_renderer().create_buffer(
//             vec![
//                 Vertex {
//                     position: [1., 1., 0.],
//                     tex_coords: [1., 1.],
//                 },
//                 Vertex {
//                     position: [-1., -1., 0.],
//                     tex_coords: [0., 0.],
//                 },
//                 Vertex {
//                     position: [1., -1., 0.],
//                     tex_coords: [1., 0.],
//                 },
//                 Vertex {
//                     position: [-1., 1., 0.],
//                     tex_coords: [0., 1.],
//                 },
//             ],
//             BufferUsages::VERTEX,
//         );
//         let index_buffer = context
//             .get_renderer()
//             .create_buffer(vec![0, 2, 1, 1, 3, 0], BufferUsages::INDEX);
//         let view_proj = glam::Mat4::from_translation(Vec3::new(0., 0., 0.));
//         let uniform_buf = context.get_renderer().create_buffer(
//             vec![Uniform {
//                 proj: view_proj.to_cols_array_2d(),
//             }],
//             BufferUsages::VERTEX,
//         );
//         let bind_group = BindGroup::new_from_layout(
//             context.get_renderer(),
//             vec![
//                 BindGroupEntryResources {
//                     binding: 0,
//                     resource: wgpu::BindingResource::TextureView(&texture.view),
//                 },
//                 BindGroupEntryResources {
//                     binding: 1,
//                     resource: wgpu::BindingResource::Sampler(&texture.sampler),
//                 },
//                 BindGroupEntryResources {
//                     binding: 2,
//                     resource: wgpu::BindingResource::Buffer(uniform_buf),
//                 },
//             ],
//             &layout,
//         );
//         Self {
//             pipeline,
//             buffer,
//             index_buffer,
//             uniform_buf,
//             texture,
//             window_res: HashMap::new(),
//             bind_group,
//         }
//     }
// }
// #[repr(C)]
// #[derive(Copy, Clone, Debug, Pod, Zeroable)]
// struct Vertex {
//     position: [f32; 3],
//     tex_coords: [f32; 2],
// }
// impl Vertex {
//     const ATTRIBS: [wgpu::VertexAttribute; 2] =
//         wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

//     fn desc() -> wgpu::VertexBufferLayout<'static> {
//         use std::mem;

//         wgpu::VertexBufferLayout {
//             array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
//             step_mode: wgpu::VertexStepMode::Vertex,
//             attributes: &Self::ATTRIBS,
//         }
//     }
// }

// #[repr(C)]
// #[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
// struct Uniform {
//     proj: [[f32; 4]; 4],
// }
// #[repr(C)]
// #[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
// struct WindowResUniform {
//     window_res: [u32; 2],
// }
// pub struct SpritesResources {
//     pipeline: PipelineId,
//     bind_group_layout: BindGroupLayout,
// }
// impl SpritesResources {
//     pub fn get_or_init<'a>(context: &'a mut AppContext) -> &'a Self {
//         if context.contains_resource::<Self>() {
//             return context.get_resource::<Self>().unwrap();
//         } else {
//             let me = Self::init(context.get_mut_renderer());

//             context.insert_resource(me);
//             context.get_resource::<Self>().unwrap()
//         }
//     }
//     pub fn init(renderer: &mut Renderer) -> Self {
//         let bind_group_layout = BindGroupLayout::new(
//             renderer,
//             vec![
//                 BindGroupEntryLayout {
//                     binding: 0,
//                     visibility: ShaderStages::FRAGMENT,
//                     ty: BindingType::Texture {
//                         multisampled: false,
//                         view_dimension: wgpu::TextureViewDimension::D2,
//                         sample_type: wgpu::TextureSampleType::Float { filterable: true },
//                     },
//                 },
//                 BindGroupEntryLayout {
//                     binding: 1,
//                     visibility: ShaderStages::FRAGMENT,
//                     ty: BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
//                 },
//                 BindGroupEntryLayout {
//                     binding: 2,
//                     visibility: ShaderStages::VERTEX_FRAGMENT,
//                     ty: wgpu::BindingType::Buffer {
//                         ty: wgpu::BufferBindingType::Uniform,
//                         has_dynamic_offset: false,
//                         min_binding_size: None,
//                     },
//                 },
//                 BindGroupEntryLayout {
//                     binding: 3,
//                     visibility: ShaderStages::VERTEX,
//                     ty: wgpu::BindingType::Buffer {
//                         ty: wgpu::BufferBindingType::Uniform,
//                         has_dynamic_offset: false,
//                         min_binding_size: None,
//                     },
//                 },
//             ],
//         );

//         let pipeline = renderer.create_pipeline(PipelineOptions {
//             vertex_shader: include_str!("sprite_shader.wgsl").to_string(),
//             vertex_entry_point: String::from("vs_main"),
//             fragment_shader: None,
//             fragment_entry_point: String::from("fs_main"),
//             frag_blend: Some(BlendState::REPLACE),
//             write_mask: ColorWrites::all(),
//             bind_group_layouts: vec![bind_group_layout.layout()],
//             buffers: vec![Vertex::desc()],
//             primitive: PrimitiveState {
//                 topology: wgpu::PrimitiveTopology::TriangleList,
//                 strip_index_format: None,
//                 front_face: wgpu::FrontFace::Ccw,
//                 cull_mode: Some(wgpu::Face::Back),
//                 // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
//                 polygon_mode: wgpu::PolygonMode::Fill,
//                 // Requires Features::DEPTH_CLIP_CONTROL
//                 unclipped_depth: false,
//                 // Requires Features::CONSERVATIVE_RASTERIZATION
//                 conservative: false,
//             },
//             depth_stencil: None,
//             multisample: MultisampleState {
//                 count: 1,
//                 mask: !0,
//                 alpha_to_coverage_enabled: false,
//             },
//             cache: None,
//         });
//         Self {
//             pipeline,
//             bind_group_layout,
//         }
//     }
// }
