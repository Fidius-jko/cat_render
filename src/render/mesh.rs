use std::collections::HashMap;

use bytemuck::{Pod, Zeroable};
use wgpu::BufferUsages;

use crate::render::bind_group::{BindGroupLayout, BindingType};

use super::{
    bind_group::{BindGroup, BindGroupEntryLayout, BindGroupEntryResources, ShaderStages},
    buffer::{Buffer, UnTypedBuffer},
    render_pipeline::{PipelineId, PipelineOptions},
    texture::Texture,
    Render, Renderer,
};

pub struct Mesh<V: Pod + Zeroable + Clone> {
    vertices: Vec<V>,
    buffer: Option<Buffer<V>>,
    indicies: Vec<u16>,
    index_buffer: Option<Buffer<u16>>,
    is_need_update_buf: bool,
    is_need_update_index_buf: bool,
}

impl<V: Pod + Zeroable> Mesh<V> {
    pub fn new(vertices: Vec<V>, indicies: Vec<u16>) -> Self {
        Self {
            vertices,
            indicies,
            buffer: None,
            index_buffer: None,
            is_need_update_buf: false,
            is_need_update_index_buf: false,
        }
    }
    pub fn update_if_need(&mut self, renderer: &Renderer) {
        if let None = self.buffer {
            self.buffer = Some(renderer.create_buffer(
                self.vertices.clone(),
                BufferUsages::VERTEX | BufferUsages::COPY_DST,
            ));
        }
        if let None = self.index_buffer {
            self.index_buffer = Some(renderer.create_buffer(
                self.indicies.clone(),
                BufferUsages::INDEX | BufferUsages::COPY_DST,
            ));
        }
        if self.is_need_update_buf {
            renderer.update_buffer(self.vertices.clone(), self.buffer.as_mut().unwrap());
        }
        if self.is_need_update_index_buf {
            renderer.update_buffer(self.indicies.clone(), self.index_buffer.as_mut().unwrap());
        }
    }
    pub fn draw_with_material(
        &mut self,
        render: &mut Render,
        material: &Material,
        material_slot: u32,
    ) {
        self.update_if_need(render.get_renderer());
        material.use_me(render, material_slot);
        render.set_vertex_buffer(self.buffer.as_ref().unwrap(), 0, ..);
        render.set_index_buffer(self.index_buffer.as_ref().unwrap(), ..);
        render.draw_indexed(
            0..self.index_buffer.as_ref().unwrap().get_vertices_number(),
            0,
            0..1,
        );
    }
}

// Uniforms and textures

/// Ignore in `PipelineOptions` field `bind_group_layouts`
pub struct MaterialLayoutBuilder {
    pipeline_options: PipelineOptions,
    uniforms: Vec<(u32, ShaderStages)>,
    textures: Vec<(u32, u32, ShaderStages)>,
}

impl MaterialLayoutBuilder {
    /// Ignore in `PipelineOptions` field `bind_group_layouts`
    pub fn new(pipeline_options: PipelineOptions) -> Self {
        Self {
            pipeline_options,
            uniforms: Vec::new(),
            textures: Vec::new(),
        }
    }
    pub fn register_uniform_at(&mut self, slot: u32, vis: ShaderStages) {
        self.uniforms.push((slot, vis));
    }
    pub fn register_texture_at(&mut self, slot: u32, sample_slot: u32, vis: ShaderStages) {
        self.textures.push((slot, sample_slot, vis));
    }
    // MAYBE TODO: support dynamic offset
    pub fn build(mut self, renderer: &mut Renderer) -> MaterialLayout {
        let mut entries = Vec::new();
        for (binding, sample_bind, vis) in self.textures {
            entries.push(BindGroupEntryLayout {
                binding,
                visibility: vis,
                ty: BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
            });
            entries.push(BindGroupEntryLayout {
                binding: sample_bind,
                visibility: vis,
                ty: BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            });
        }
        for (binding, vis) in self.uniforms {
            entries.push(BindGroupEntryLayout {
                binding,
                visibility: vis,
                ty: BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            });
        }
        let bind_group_layout = BindGroupLayout::new(&renderer, entries);

        self.pipeline_options.bind_group_layouts = vec![bind_group_layout.layout()];
        let pipeline = renderer.create_pipeline(self.pipeline_options);

        MaterialLayout {
            // uniforms: self.uniforms.clone(),
            // textures: self.textures.clone(),
            pipeline,
            bindgroup: bind_group_layout,
        }
    }
}

pub struct MaterialLayout {
    // uniforms: Vec<u32>,
    // textures: Vec<(u32, u32)>,
    pipeline: PipelineId,
    bindgroup: BindGroupLayout,
}

pub struct Material {
    pipeline: PipelineId,
    bindgroup: BindGroup,
    uniform_buffers: HashMap<u32, UnTypedBuffer>,
}
impl Material {
    /// Uniform is bytes!
    pub fn from_layout(
        renderer: &Renderer,
        layout: &MaterialLayout,
        uniforms: Vec<(u32, Vec<u8>)>,
        textures: Vec<(u32, u32, Texture)>,
    ) -> Self {
        let mut res = Vec::new();
        let mut uniform_buffers: HashMap<u32, UnTypedBuffer> = HashMap::new();
        for (binding, bytes) in uniforms.iter() {
            let buf = UnTypedBuffer::new(
                renderer,
                vec![bytes.clone()],
                BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            );
            uniform_buffers.insert(binding.clone(), buf);
        }

        for (binding, bytes) in uniforms.iter() {
            res.push(BindGroupEntryResources {
                binding: binding.clone(),
                resource: uniform_buffers
                    .get(&binding)
                    .unwrap()
                    .as_entire_binding()
                    .clone(),
            });
        }
        for (binding, sample_binding, texture) in textures.iter() {
            res.push(BindGroupEntryResources {
                binding: *binding,
                resource: wgpu::BindingResource::TextureView(&texture.view),
            });
            res.push(BindGroupEntryResources {
                binding: sample_binding.clone(),
                resource: wgpu::BindingResource::Sampler(&texture.sampler),
            });
        }
        let bind_group = BindGroup::new_from_layout(renderer, res, &layout.bindgroup);
        Self {
            bindgroup: bind_group,
            pipeline: layout.pipeline.clone(),
            uniform_buffers,
        }
    }
    pub fn update_uniform(&mut self, slot: u32, bytes: Vec<u8>, renderer: &Renderer) {
        self.uniform_buffers
            .get_mut(&slot)
            .expect("No uniform on slot!")
            .update(renderer, vec![bytes]);
    }
    // pub fn change_texture() {} TODO
    pub fn use_me(&self, render: &mut Render, slot: u32) {
        render.set_pipeline(self.pipeline.clone());
        render.set_bind_group(slot, &self.bindgroup, &[]);
    }
}
