pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl Color {
    pub fn srgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
    pub fn srgba_255(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: r / 255.,
            g: g / 255.,
            b: b / 255.,
            a: a / 255.,
        }
    }
    pub fn srgb_255(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: r / 255.,
            g: g / 255.,
            b: b / 255.,
            a: 1.0,
        }
    }
    pub fn srgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }
}
impl Into<wgpu::Color> for Color {
    fn into(self) -> wgpu::Color {
        wgpu::Color {
            r: self.r as f64,
            g: self.g as f64,
            b: self.b as f64,
            a: self.a as f64,
        }
    }
}
