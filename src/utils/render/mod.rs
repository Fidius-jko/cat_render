use crate::context::AppContext;

pub mod sprite;
pub mod texture_atlas;

pub fn init_render_utils(context: &mut AppContext) {
    sprite::init_sprites(context);
}
