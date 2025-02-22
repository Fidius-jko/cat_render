use glam::Vec2;

use crate::render::small::Rect;

pub struct TextureAtlas {
    textures: Vec<Rect>,
}
impl TextureAtlas {
    pub fn from_gird(size: Vec2, rows: usize, columns: usize) -> Self {
        let mut textures = Vec::new();
        for row in 0..rows {
            for column in 0..columns {
                textures.push(Rect::new(
                    column as f32 * size.x,
                    row as f32 * size.y,
                    (column as f32 + 1.) * size.x,
                    (row as f32 + 1.) * size.y,
                ));
            }
        }
        Self { textures }
    }
    pub fn new() -> Self {
        TextureAtlas {
            textures: Vec::new(),
        }
    }
    pub fn add_texture(&mut self, rect: Rect) -> usize {
        self.textures.push(rect);
        self.textures.len() - 1
    }
    pub fn get_texture(&self, id: usize) -> Option<Rect> {
        self.textures.get(id).cloned()
    }
}
