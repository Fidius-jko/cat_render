use std::marker::PhantomData;

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use super::Renderer;

pub use wgpu::BindingResource;
pub use wgpu::BufferUsages;

pub struct Buffer<V: bytemuck::Pod + bytemuck::Zeroable> {
    pub(crate) wgpu_buffer: wgpu::Buffer,
    pub(crate) vertices_number: u32,
    mark: PhantomData<V>,
}

impl<V: Pod + Zeroable> Buffer<V> {
    pub(crate) fn new(renderer: &Renderer, vertices: Vec<V>, usage: BufferUsages) -> Self {
        let buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage,
            });
        Self {
            wgpu_buffer: buffer,
            vertices_number: vertices.len() as u32,
            mark: PhantomData::default(),
        }
    }
    pub(crate) fn update(&self, renderer: &Renderer, vertices: Vec<V>) {
        renderer
            .queue
            .write_buffer(&self.wgpu_buffer, 0, bytemuck::cast_slice(&vertices));
    }
    pub fn get_vertices_number(&self) -> u32 {
        self.vertices_number
    }
    pub fn as_entire_binding(&self) -> BindingResource {
        self.wgpu_buffer.as_entire_binding()
    }
}
