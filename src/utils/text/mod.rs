pub mod font;
pub use font::*;
// use glam::Vec2;

// use crate::{
//     context::AppContext,
//     render::{bind_group::BindGroup, Color},
// };

// use super::render::sprite::{Sprite, SpriteLayout};

// pub struct TextRenderLayout {
//     sprite_layout: SpriteLayout,
// }
// impl TextRenderLayout {
//     pub fn new(context: &mut AppContext, camera: BindGroup) -> Self {
//         Self {
//             sprite_layout: SpriteLayout::new(context, camera, Some(Vec2::splat(0.))),
//         }
//     }
// }
// pub struct Text {
//     text: String,
//     text_image: TextImage,
//     is_changed: bool,
//     sprite: Option<Sprite>,
// }
// impl Text {
//     pub fn new(
//         layout: TextRenderLayout,
//         text: &str,
//         font_size: f32,
//         line_h: f32,
//         max: Option<Vec2>,
//         color: Color,
//     ) -> Self {
//         let max = if let Some(v) = max {
//             v
//         } else {
//             Vec2::splat(f32::MAX)
//         };
//         let image = TextImage::new(text, font_size, line_h, max, color);

//         Self {
//             text: text.into(),
//             text_image: image,
//             is_changed: true,
//             sprite: Sprite::new(layout, width, height, transform, texture, rect),
//         }
//     }
// }
