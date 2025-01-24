// IT IS ONLY EXAMPLE
// use std::sync::Arc;

// use wgpu::{BufferUsages, ShaderStages};

// use super::{
//     bind_group::{BindGroup, BindGroupEntry},
//     buffer::Buffer,
//     Render, Renderer,
// };

// pub struct Uniform<V: bytemuck::Pod + bytemuck::Zeroable> {
//     buf: Buffer<V>,
//     bind_group: BindGroup,
// }
// impl<V: bytemuck::Pod + bytemuck::Zeroable> Uniform<V> {
//     pub fn new(renderer: &Renderer, value: V, visibility: ShaderStages) -> Self {
//         let buf =
//             renderer.create_buffer(vec![value], BufferUsages::UNIFORM | BufferUsages::COPY_DST);
//         let bind_group = renderer.create_bind_group(vec![BindGroupEntry {
//             binding: 0,
//             visibility,
//             ty: wgpu::BindingType::Buffer {
//                 ty: wgpu::BufferBindingType::Uniform,
//                 has_dynamic_offset: false,
//                 min_binding_size: None,
//             },
//             resource: buf.as_entire_binding(),
//         }]);
//         Self { buf, bind_group }
//     }
//     pub fn bind_group_layout(&self) -> Arc<wgpu::BindGroupLayout> {
//         self.bind_group.layout()
//     }
//     pub fn use_for_render(&self, render: &mut Render, index: u32) {
//         render.set_bind_group(index, &self.bind_group, &[]);
//     }
//     pub fn update(&self, renderer: &Renderer, value: V) {
//         renderer.update_buffer(vec![value], &self.buf);
//     }
// }
// pub fn set_uniform<V: Pod + Zeroable>(&mut self, uniform: &Uniform<V>, index: u32) {
//     uniform.use_for_render(self, index);
// }
// pub fn create_uniform<V: Pod + Zeroable>(
//     &self,
//     value: V,
//     visibility: ShaderStages,
// ) -> Uniform<V> {
//     Uniform::new(self, value, visibility)
// }
// pub fn update_uniform<V: Pod + Zeroable>(&self, value: V, uniform: &Uniform<V>) {
//     uniform.update(self, value);
// }
