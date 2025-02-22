use std::ops::{Add, Mul};

use glam::{Mat4, Quat, Vec2, Vec3, Vec4};

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

impl From<Color> for wgpu::Color {
    fn from(val: Color) -> Self {
        wgpu::Color {
            r: val.r as f64,
            g: val.g as f64,
            b: val.b as f64,
            a: val.a as f64,
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
        Mat4::from_scale_rotation_translation(
            self.scale,
            Quat::from_rotation_x(self.rotation.x)
                * Quat::from_rotation_y(self.rotation.y)
                * Quat::from_rotation_z(self.rotation.z),
            self.translation,
        )
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
    pub fn is_inserction(&self, rect: Rect) -> bool {
        (self.max.x >= rect.min.x && self.min.x <= rect.max.x)
            && (self.max.y >= rect.min.y && self.min.y <= rect.max.y)
    }
    pub fn transformed(&self, transform: Transform) -> Self {
        let matrix = transform.get_matrix();
        let p1 = matrix.mul_vec4(Vec4::new(self.min.x, self.min.y, 0., 0.));
        let p2 = matrix.mul_vec4(Vec4::new(self.max.x, self.min.y, 0., 0.));
        let p3 = matrix.mul_vec4(Vec4::new(self.min.x, self.max.y, 0., 0.));
        let p4 = matrix.mul_vec4(Vec4::new(self.max.x, self.max.y, 0., 0.));

        let x = [p1.x, p2.x, p3.x, p4.x];
        let y = [p1.y, p2.y, p3.y, p4.y];
        let mut min = Vec2::splat(f32::INFINITY);
        let mut max = Vec2::splat(-f32::INFINITY);
        for x in x {
            let x = x + transform.translation.x;
            if x < min.x {
                min.x = x;
            }
            if x > max.x {
                max.x = x;
            }
        }
        for y in y {
            let y = y + transform.translation.y;
            if y < min.y {
                min.y = y;
            }
            if y > max.y {
                max.y = y;
            }
        }
        Self { min, max }
    }
}
impl Mul<Vec2> for Rect {
    type Output = Rect;
    fn mul(self, rhs: Vec2) -> Self::Output {
        let mut rect = self;
        rect.min *= rhs;
        rect.max *= rhs;
        rect
    }
}
impl Add<Vec2> for Rect {
    type Output = Rect;
    fn add(self, rhs: Vec2) -> Self::Output {
        let mut rect = self;
        rect.min += rhs;
        rect.max += rhs;
        rect
    }
}
