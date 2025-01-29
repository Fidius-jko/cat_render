/// ---------Buffer------
/// see learn wgpu: buffers
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use bytemuck::{Pod, Zeroable};
use wgpu::{util::DeviceExt, BindingResource, BufferUsages};

use super::Renderer;

/// Buffer with V as Vertex
#[derive(Clone)]
pub struct Buffer<V: bytemuck::Pod + bytemuck::Zeroable> {
    pub(crate) wgpu_buffer: wgpu::Buffer,
    vertices_number: Arc<Mutex<u32>>,
    mark: PhantomData<V>,
}

impl<V: Pod + Zeroable> Buffer<V> {
    /// Creates new buffer
    pub fn new(renderer: &Renderer, vertices: Vec<V>, usage: BufferUsages) -> Self {
        let buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage,
            });
        Self {
            wgpu_buffer: buffer,
            vertices_number: Arc::new(Mutex::new(vertices.len() as u32)),
            mark: PhantomData::default(),
        }
    }
    /// Update buffer
    /// PANICS if usage is not BufferUsages::COPY_DST
    pub fn update(&mut self, renderer: &Renderer, vertices: Vec<V>) {
        *self.vertices_number.lock().unwrap() = vertices.len() as u32;
        renderer
            .queue
            .write_buffer(&self.wgpu_buffer, 0, bytemuck::cast_slice(&vertices));
    }
    /// Number of vertices
    pub fn get_vertices_number(&self) -> u32 {
        self.vertices_number.lock().unwrap().clone()
    }
    /// Need if using it as uniform
    pub fn as_entire_binding(&self) -> BindingResource {
        self.wgpu_buffer.as_entire_binding()
    }
}
/// Buffer with only bytes
pub struct UnTypedBuffer {
    pub(crate) wgpu_buffer: wgpu::Buffer,
    pub(crate) vertices_number: Arc<Mutex<u32>>,
}

impl UnTypedBuffer {
    /// See Buffer
    pub fn new(renderer: &Renderer, vertices_bytes: Vec<Vec<u8>>, usage: BufferUsages) -> Self {
        let buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Buffer"),
                contents: &vertices_bytes
                    .iter()
                    .cloned()
                    .flatten()
                    .collect::<Vec<u8>>(),
                usage,
            });
        Self {
            wgpu_buffer: buffer,
            vertices_number: Arc::new(Mutex::new(vertices_bytes.len() as u32)),
        }
    }
    /// See Buffer
    pub fn update(&mut self, renderer: &Renderer, vertices_bytes: Vec<Vec<u8>>) {
        *self.vertices_number.lock().unwrap() = vertices_bytes.len() as u32;
        renderer.queue.write_buffer(
            &self.wgpu_buffer,
            0,
            &vertices_bytes
                .iter()
                .cloned()
                .flatten()
                .collect::<Vec<u8>>(),
        );
    }
    /// See Buffer
    pub fn get_vertices_number(&self) -> u32 {
        self.vertices_number.lock().unwrap().clone()
    }
    /// See Buffer
    pub fn as_entire_binding(&self) -> BindingResource {
        self.wgpu_buffer.as_entire_binding()
    }
}
