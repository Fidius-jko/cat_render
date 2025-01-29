pub struct Camera {}

pub struct View {}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ViewUniform {
    proj: [[f32; 4]; 4],
}
