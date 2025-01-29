pub use bytemuck;
pub use wgpu;

pub mod bind_group;
pub mod buffer;
pub mod camera;
pub mod color;
pub mod mesh;
pub mod render_pipeline;
pub mod surface;
pub mod texture;

pub use color::Color;

use bind_group::BindGroup;
use bind_group::{BindGroupEntryLayout, BindGroupEntryResources};
use buffer::Buffer;
use image::DynamicImage;
use render_pipeline::{PipelineId, PipelineOptions, Pipelines};
use surface::{SurfaceId, Surfaces};
use texture::Texture;

use std::{
    mem::replace,
    ops::{Range, RangeBounds},
    rc::Rc,
    sync::Arc,
};

use bytemuck::{Pod, Zeroable};
use wgpu::{
    Adapter, BufferUsages, Device, DynamicOffset, IndexFormat, Instance, Queue, RenderPass,
    RenderPipeline, Surface, SurfaceTexture, TextureFormat, TextureView,
};
use winit::{
    dpi::PhysicalSize,
    window::{Window, WindowId},
};

pub struct Renderer {
    pipelines: Pipelines,
    instance: Instance,
    pub(crate) device: Device,
    pub(crate) queue: Queue,
    pub(crate) adapter: Adapter,
    pub(crate) needs_exit: bool,
}

/// Rendering is here
#[allow(dead_code)]
pub struct Render<'a> {
    output: &'a SurfaceTexture,
    view: TextureView,
    render_pass: RenderPass<'a>,
    renderer: &'a mut Renderer,
    surface_id: SurfaceId,
}

impl<'a> Render<'a> {
    /// Get renderer for init something
    pub fn get_renderer(&self) -> &Renderer {
        &self.renderer
    }
    /// Get surface size
    pub fn get_surface_size(&self) -> (u32, u32) {
        let size = self.output.texture.size();
        (size.width, size.height)
    }
    /// Get surface id
    pub fn get_surface_id(&self) -> SurfaceId {
        self.surface_id.clone()
    }

    /// Set bind group
    pub fn set_bind_group(
        &mut self,
        index: u32,
        bind_group: &BindGroup,
        offsets: &[DynamicOffset],
    ) {
        self.render_pass
            .set_bind_group(index, &bind_group.group, offsets);
    }
    /// Set pipeline
    pub fn set_pipeline(&mut self, id: PipelineId) {
        self.render_pass
            .set_pipeline(&self.renderer.get_pipeline(self.output.texture.format(), id));
    }
    /// draw
    pub fn draw(&mut self, vertices: Range<u32>, instances: Range<u32>) {
        self.render_pass.draw(vertices, instances);
    }
    /// draw with indicies
    pub fn draw_indexed(&mut self, vertices: Range<u32>, base_vertex: i32, instances: Range<u32>) {
        self.render_pass
            .draw_indexed(vertices, base_vertex, instances);
    }
    /// set vertex buffer
    pub fn set_vertex_buffer<V: Zeroable + Pod>(
        &mut self,
        buffer: &Buffer<V>,
        slot: u32,
        buffer_slice: impl RangeBounds<u64>,
    ) {
        if buffer.wgpu_buffer.usage() & BufferUsages::VERTEX == BufferUsages::empty() {
            log::warn!("Buffer is not for vertex!");
            log::warn!("Buffer is not added!");
            return;
        }
        self.render_pass
            .set_vertex_buffer(slot, buffer.wgpu_buffer.slice(buffer_slice));
    }
    /// set index buffer need for `draw_indexed`
    pub fn set_index_buffer<V: Zeroable + Pod + GetIndexFormat>(
        &mut self,
        buffer: &Buffer<V>,
        buffer_slice: impl RangeBounds<u64>,
    ) {
        if buffer.wgpu_buffer.usage() & BufferUsages::INDEX == BufferUsages::empty() {
            log::warn!("Buffer is not for index!");
            log::warn!("Buffer is not added!");
            return;
        }
        self.render_pass.set_index_buffer(
            buffer.wgpu_buffer.slice(buffer_slice),
            V::get_index_format(),
        );
    }
}

pub trait GetIndexFormat {
    fn get_index_format() -> IndexFormat;
}

impl GetIndexFormat for u16 {
    fn get_index_format() -> IndexFormat {
        IndexFormat::Uint16
    }
}
impl GetIndexFormat for u32 {
    fn get_index_format() -> IndexFormat {
        IndexFormat::Uint32
    }
}

impl Renderer {
    /// Create bind group
    pub fn create_bind_group(
        &self,
        layout: Vec<BindGroupEntryLayout>,
        res: Vec<BindGroupEntryResources>,
    ) -> BindGroup {
        BindGroup::new(self, layout, res)
    }
    /// Create texture
    pub fn create_texture_from_bytes(&mut self, bytes: &[u8]) -> Result<Texture, anyhow::Error> {
        Texture::from_bytes(self, bytes)
    }
    /// Create texture from `image` crate
    pub fn create_texture_from_image(
        &mut self,
        img: &DynamicImage,
    ) -> Result<Texture, anyhow::Error> {
        Texture::from_image(self, img)
    }

    pub(crate) fn create_surface(&mut self, window: Arc<Window>) -> SurfaceId {
        Surfaces::get().create_surface(self, window)
    }
    pub(crate) fn get_surface(&self, id: SurfaceId) -> Arc<Surface<'_>> {
        Surfaces::get().get_surface(id).wgpu_surface.clone()
    }
    pub(crate) fn on_resize(&mut self, window_id: &WindowId, new_size: PhysicalSize<u32>) {
        Surfaces::get().resize_window_surface(self, window_id, new_size);
    }

    /// Update buffer
    pub fn update_buffer<V: Pod + Zeroable>(&self, vertices: Vec<V>, buffer: &mut Buffer<V>) {
        buffer.update(self, vertices);
    }
    /// Create buffer
    pub fn create_buffer<V: Pod + Zeroable>(
        &self,
        vertices: Vec<V>,
        usage: BufferUsages,
    ) -> Buffer<V> {
        Buffer::<V>::new(self, vertices, usage)
    }
    /// Is exists surface
    pub fn exists_surface(&self, surface: SurfaceId) -> bool {
        Surfaces::get().exists(surface)
    }
    /// Create pipeline
    pub fn create_pipeline(&mut self, options: PipelineOptions) -> PipelineId {
        let mut pipelines = replace(&mut self.pipelines, Pipelines::new());
        let id = pipelines.create_pipeline(&self, options);
        let _ = replace(&mut self.pipelines, pipelines);
        return id;
    }
    /// Get pipeline
    pub fn get_pipeline(
        &mut self,
        format: TextureFormat,
        pipeline_id: PipelineId,
    ) -> Rc<RenderPipeline> {
        let mut pipelines = replace(&mut self.pipelines, Pipelines::new());
        let pipeline = pipelines.get_pipeline_for_surface(format, pipeline_id, self);
        let _ = replace(&mut self.pipelines, pipelines);
        return pipeline;
    }

    /// Renderings starts here!
    #[allow(unused_assignments)]
    pub fn start_render_for_surface(
        &mut self,
        surface_id: SurfaceId,
        clear_color: Option<Color>,
        mut commands_sender: impl FnMut(&mut Render),
    ) {
        if !self.exists_surface(surface_id.clone()) {
            log::error!("Surface doesn't exists {:?}", surface_id);
            log::warn!("Render stoped.");
            return;
        }
        if self.needs_exit {
            return;
        }
        let mut needs_exit = false;

        let surface = self.get_surface(surface_id.clone());
        let output = match surface.get_current_texture() {
            Ok(o) => o,
            Err(e) => {
                match e {
                    // Reconfigure the surface if it's lost or outdated
                    wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated => {
                        // TODO state.resize(state.size)
                    }
                    // The system is out of memory, we should probably quit
                    wgpu::SurfaceError::OutOfMemory => {
                        log::error!("Out of memory :(");
                        needs_exit = true;
                    }

                    // This happens when the a frame takes too long to present
                    wgpu::SurfaceError::Timeout => {
                        log::warn!("Surface timeout")
                    }
                    wgpu::SurfaceError::Other => {}
                }
                log::error!("Failed to start render: {}", e);
                return;
            }
        };
        drop(surface);
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let load;
            match clear_color {
                Some(c) => {
                    load = wgpu::LoadOp::Clear(c.into());
                }
                None => load = wgpu::LoadOp::Load,
            }
            let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            let mut render = Render {
                output: &output,
                view,
                render_pass,
                renderer: self,
                surface_id: surface_id,
            };
            (commands_sender)(&mut render);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        self.needs_exit = needs_exit;
    }

    pub(crate) async fn new() -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::NOT_SUPPPORT,
            ..Default::default()
        });
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web, we'll have to disable some.
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                    memory_hints: Default::default(),
                },
                None, // Trace path
            )
            .await
            .unwrap();
        Self {
            instance,
            device,
            queue,
            adapter,
            needs_exit: false,
            pipelines: Pipelines::new(),
        }
    }
}
