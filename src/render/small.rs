use glam::{Mat4, Quat, Vec2, Vec3};

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

#[derive(Default, Clone, Copy)]
pub struct Transform {
    pub translation: Vec3,
    pub scale: Vec3,
    pub rotation: Vec3,
}

impl Transform {
    pub fn from_translation(trans: Vec3) -> Self {
        Self {
            translation: trans,
            scale: Vec3::splat(1.),
            ..Default::default()
        }
    }
    pub fn from_scale(scale: Vec3) -> Self {
        Self {
            scale,
            ..Default::default()
        }
    }
    pub fn from_rotation(rot: Vec3) -> Self {
        Self {
            rotation: rot,
            scale: Vec3::splat(1.),
            ..Default::default()
        }
    }
    pub fn get_matrix(&self) -> Mat4 {
        let mat = Mat4::from_scale_rotation_translation(
            self.scale,
            Quat::from_rotation_x(self.rotation.x)
                * Quat::from_rotation_y(self.rotation.y)
                * Quat::from_rotation_z(self.rotation.z),
            self.translation,
        );
        mat
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
}
impl Rect {
    pub fn new(x: f32, y: f32, x2: f32, y2: f32) -> Self {
        Self {
            min: Vec2::new(x, y),
            max: Vec2::new(x2, y2),
        }
    }
}
