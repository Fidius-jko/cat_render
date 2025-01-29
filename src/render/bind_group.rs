use std::sync::Arc;

use super::Renderer;

#[derive(Clone)]
pub struct BindGroup {
    pub(crate) group: wgpu::BindGroup,
    layout: Arc<wgpu::BindGroupLayout>,
}
#[derive(Clone)]
pub struct BindGroupLayout {
    layout: Arc<wgpu::BindGroupLayout>,
}

impl BindGroupLayout {
    pub fn new(renderer: &Renderer, entries: Vec<BindGroupEntryLayout>) -> Self {
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

        Self {
            layout: Arc::new(bind_group_layout),
        }
    }
    pub fn layout(&self) -> Arc<wgpu::BindGroupLayout> {
        self.layout.clone()
    }
}

impl BindGroup {
    pub fn new_from_layout(
        renderer: &Renderer,
        resources: Vec<BindGroupEntryResources>,
        layout: &BindGroupLayout,
    ) -> Self {
        let mut out_entries = Vec::new();
        for entry in resources {
            out_entries.push(wgpu::BindGroupEntry {
                binding: entry.binding,
                resource: entry.resource,
            });
        }
        let bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &layout.layout,
                entries: &out_entries,
            });
        Self {
            group: bind_group,
            layout: layout.layout.clone(),
        }
    }
    pub fn cloned_with_new_info(
        &self,
        renderer: &Renderer,
        resources: Vec<BindGroupEntryResources>,
    ) -> Self {
        let mut out_entries = Vec::new();
        for entry in resources {
            out_entries.push(wgpu::BindGroupEntry {
                binding: entry.binding,
                resource: entry.resource,
            });
        }
        let bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &self.layout,
                entries: &out_entries,
            });
        Self {
            group: bind_group,
            layout: self.layout.clone(),
        }
    }
    pub fn new(
        renderer: &Renderer,
        layout: Vec<BindGroupEntryLayout>,
        resources: Vec<BindGroupEntryResources>,
    ) -> Self {
        Self::new_from_layout(renderer, resources, &BindGroupLayout::new(renderer, layout))
    }
    pub fn layout(&self) -> Arc<wgpu::BindGroupLayout> {
        self.layout.clone()
    }
}

pub struct BindGroupEntryResources<'a> {
    pub binding: u32,
    pub resource: wgpu::BindingResource<'a>,
}
pub struct BindGroupEntryLayout {
    pub binding: u32,
    pub visibility: wgpu::ShaderStages,
    pub ty: wgpu::BindingType,
}

pub type BindingResource<'a> = wgpu::BindingResource<'a>;
pub type BindingType = wgpu::BindingType;
pub type ShaderStages = wgpu::ShaderStages;
