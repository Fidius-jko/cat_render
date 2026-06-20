use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex, MutexGuard},
};

use cosmic_text::{fontdb::Source, Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use glam::Vec2;
use image::DynamicImage;

use crate::render::{texture::Texture, Color};

struct GlobalFontResources {
    pub swash_cache: SwashCache,
    pub system_font: Font,
}
impl GlobalFontResources {
    fn new() -> Self {
        Self {
            swash_cache: SwashCache::new(),
            system_font: Font::system_font(),
        }
    }
    pub fn get_mut() -> MutexGuard<'static, Self> {
        log::trace!(
            "GLOBAL FONT RESOURCES WRITE {:?}",
            std::thread::current().id()
        );
        GLOBAL_FONT_RES.lock().unwrap()
    }
}
static GLOBAL_FONT_RES: LazyLock<Mutex<GlobalFontResources>> =
    LazyLock::new(|| Mutex::new(GlobalFontResources::new()));

pub struct Font {
    system: FontSystem,
    buffer: Buffer,
}

impl Font {
    fn system_font() -> Self {
        let mut sys = FontSystem::new();
        let buf = Buffer::new(&mut sys, Metrics::new(1., 1.));
        Self {
            system: FontSystem::new(),
            buffer: buf,
        }
    }
    pub fn new(source: Source) -> Self {
        let mut sys = FontSystem::new_with_fonts(vec![source]);
        let buf = Buffer::new(&mut sys, Metrics::new(1., 1.));
        Self {
            system: FontSystem::new(),
            buffer: buf,
        }
    }
    pub fn render_to_new_texture(
        &mut self,
        text: &str,
        font_size: f32,
        line_h: f32,
        max: Vec2,
        color: Color,
        font_attrs: Attrs,
    ) -> Texture {
        Texture::from_image(
            &self.render_to_image(text, font_size, line_h, max, color, font_attrs),
            wgpu::FilterMode::Linear,
        )
        .unwrap()
    }

    pub fn render_to_image(
        &mut self,
        text: &str,
        font_size: f32,
        line_h: f32,
        max: Vec2,
        color: Color,
        font_attrs: Attrs,
    ) -> DynamicImage {
        let mut res = GlobalFontResources::get_mut();
        let Font { system, buffer } = self;
        buffer.set_metrics(system, Metrics::new(font_size, line_h));
        let mut buffer = buffer.borrow_with(system);
        let width = max.x;
        let height = max.y;
        buffer.set_size(Some(width), Some(height));

        let text = text.to_string();
        buffer.set_text(&text, font_attrs, Shaping::Advanced);
        buffer.shape_until_scroll(true);
        let text_color = cosmic_text::Color::rgba(
            (color.r * 255.) as u8,
            (color.g * 255.) as u8,
            (color.b * 255.) as u8,
            (color.a * 255.) as u8,
        );

        let width = 10.;
        // let height = line_h * buffer.layout_runs().count() as f32;
        let height = 10.;
        let mut canvas = vec![vec![None::<(u8, u8, u8, u8)>; height as usize]; width as usize];

        buffer.draw(&mut res.swash_cache, text_color, |x, y, w, h, color| {
            let a = color.a();
            if a == 0 || x < 0 || y < 0 || y >= height as i32 || w != 1 || h != 1 {
                // Ignore alphas of 0, or invalid x, y coordinates, or unimplemented sizes
                return;
            }
            // if x as usize > canvas.len() - 1 {
            //     let need = x as usize - canvas.len() + 1;
            //     for _ in 0..need {
            //         canvas.push(vec![None; height as usize]);
            //     }
            // }

            let r = color.r();
            let g = color.g();
            let b = color.b();
            // canvas[x as usize][y as usize] = Some((r, g, b, a));
        });
        let mut img =
            DynamicImage::new(canvas.len() as u32, height as u32, image::ColorType::Rgba8);
        // for (x, column) in canvas.iter().enumerate() {
        //     for (y, pixel) in column.iter().enumerate() {
        //         let (r, g, b, a) = pixel.unwrap_or((0, 0, 0, 0));
        //         match &mut img {
        //             DynamicImage::ImageRgba8(img) => {
        //                 img.get_pixel_mut(x as u32, y as u32).0 = [r, g, b, a];
        //             }
        //             _ => panic!("AAaA (in text render :< )"),
        //         }
        //     }
        // }
        img
    }
}

pub fn default_max() -> Vec2 {
    Vec2::splat(f32::MAX)
}
pub fn line_height(font_size: f32, add: f32) -> f32 {
    font_size + add
}
