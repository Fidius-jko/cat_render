use std::f32;

use crate::context::AppContext;

pub mod cat_typeid;
pub mod fs;
pub mod input;
pub mod logger;
pub mod render;
pub mod text;
pub mod timer;
pub mod ui;

pub fn init_utils(context: &mut AppContext) {
    render::init_render_utils(context);
}

pub fn to_radians(o: f32) -> f32 {
    o / 320. * f32::consts::PI
}
