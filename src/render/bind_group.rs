use std::sync::Arc;

use super::Renderer;

/// Bind group from wgpu BindGroup
#[derive(Clone)]
pub struct BindGroup {
    /// Inner
    pub(crate) group: wgpu::BindGroup,
    /// layout
    layout: Arc<wgpu::BindGroupLayout>,
}

/// Bind group layout from wgpu BindGroupLayout
#[derive(Clone)]
pub struct BindGroupLayout {
    layout: Arc<wgpu::BindGroupLayout>,
}

impl BindGroupLayout {
    /// Creates new bind group layout use for reuse pipelines.
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
    /// Inner layout
    pub fn layout(&self) -> Arc<wgpu::BindGroupLayout> {
        self.layout.clone()
    }
}

impl BindGroup {
    /// Creates bind group
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
    /// Creates bind group with new layout
    pub fn new(
        renderer: &Renderer,
        layout: Vec<BindGroupEntryLayout>,
        resources: Vec<BindGroupEntryResources>,
    ) -> Self {
        Self::new_from_layout(renderer, resources, &BindGroupLayout::new(renderer, layout))
    }
    /// Inner layout
    pub fn layout(&self) -> Arc<wgpu::BindGroupLayout> {
        self.layout.clone()
    }
    /// Bind group layout
    pub fn cat_layout(&self) -> BindGroupLayout {
        BindGroupLayout {
            layout: self.layout.clone(),
        }
    }
}

pub struct BindGroupEntryResources<'a> {
    /// @group(n) @binding from shader
    ///            -------
    pub binding: u32,
    /// see inner
    pub resource: wgpu::BindingResource<'a>,
}
pub struct BindGroupEntryLayout {
    /// @group(n) @binding from shader
    ///            -------
    pub binding: u32,
    /// Which shader can use this bindgroup
    pub visibility: wgpu::ShaderStages,
    /// Type of resource
    pub ty: wgpu::BindingType,
}
