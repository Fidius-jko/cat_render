use crate::context::AppContext;

pub mod sprite;

pub fn init_render_utils(context: &mut AppContext) {
    sprite::init_sprites(context);
}
