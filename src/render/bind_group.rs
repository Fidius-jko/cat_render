use std::sync::Arc;

use super::Renderer;

pub struct BindGroup {
    pub(crate) group: wgpu::BindGroup,
    layout: Arc<wgpu::BindGroupLayout>,
}

impl BindGroup {
    pub(crate) fn new(renderer: &mut Renderer, entries: Vec<BindGroupEntry>) -> Self {
        let mut out_entries = Vec::new();
        for entry in entries.iter() {
            out_entries.push(wgpu::BindGroupLayoutEntry {
                binding: entry.binding,
                visibility: entry.visibility,
                ty: entry.ty,
                count: None,
            });
        }
        let bind_group_layout =
            renderer
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &out_entries,
                    label: None,
                });
        let mut out_entries = Vec::new();
        for entry in entries {
            out_entries.push(wgpu::BindGroupEntry {
                binding: entry.binding,
                resource: entry.resource,
            });
        }
        let bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &bind_group_layout,
                entries: &out_entries,
            });
        return Self {
            group: bind_group,
            layout: Arc::new(bind_group_layout),
        };
    }
    pub fn layout(&self) -> Arc<wgpu::BindGroupLayout> {
        self.layout.clone()
    }
}

pub struct BindGroupEntry<'a> {
    pub binding: u32,
    pub visibility: wgpu::ShaderStages,
    pub ty: wgpu::BindingType,
    pub resource: wgpu::BindingResource<'a>,
}

pub type BindingResource<'a> = wgpu::BindingResource<'a>;
pub type BindingType = wgpu::BindingType;
pub type ShaderStages = wgpu::ShaderStages;
