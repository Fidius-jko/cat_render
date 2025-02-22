use std::time::{Duration, Instant};

use cat_render::{
    prelude::*,
    render::{
        camera::{Camera2D, Camera2DOptions},
        small::{Rect, Transform},
        texture::Texture,
    },
    utils::{
        fs::Filesystem,
        input::Input,
        render::sprite::{Sprite, SpriteLayout},
        timer::Timer,
    },
};
use winit::keyboard::KeyCode;

fn main() {
    let _ = App::run();
}
pub const TICK_SECS: f32 = 1. / 60.;
pub struct App {
    sprites: Vec<Sprite>,
    camera: Camera2D,
    input: Input,
    timer: Timer,
    fps_cnt: u32,
    last_i: usize,
    layout: SpriteLayout,
    texture: Texture,
}

impl CatApp for App {
    fn config() -> AppConfig {
        AppConfig {
            loop_type: LoopType::Waiting,
        }
    }
    fn new(context: &mut AppContext) -> Self {
        cat_render::utils::init_utils(context);
        context.set_fps(60);
        let window =
            context.create_window(WindowAttributes::default().with_title("Objects example"));
        let surface = context.create_surface_for_window(&window).unwrap();
        let camera = Camera2D::new(Camera2DOptions {
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            surface: surface.clone(),
            viewport_origin: Vec2::new(0.5, 0.5),
            ..Default::default()
        });
        let texture = Texture::from_bytes(
            &Filesystem::get().read("assets/happy-tree2.png").unwrap(),
            wgpu::FilterMode::Nearest,
        )
        .unwrap();
        const SIZE: usize = 2;
        let mut sprites = Vec::with_capacity(SIZE);
        let sprite_layout = SpriteLayout::new(context, camera.get_bind_group());
        for i in 0..SIZE {
            let sprite = Sprite::new(
                &sprite_layout,
                100.,
                100.,
                Transform {
                    rotation: Vec3::new(0., 0., 0.),
                    scale: Vec3::splat(4.),
                    translation: Vec3::new(50. - i as f32 * 100., 50., 0.),
                    ..Default::default()
                },
                texture.clone(),
                None,
            );

            sprites.push(sprite);
        }
        Self {
            camera,
            sprites,
            input: Input::new(),
            timer: Timer::new(Duration::from_secs_f32(1.)),
            fps_cnt: 0,
            last_i: SIZE - 1,
            layout: sprite_layout,
            texture,
        }
    }
    fn update(&mut self, _context: &mut AppContext, delta: f32) {
        self.last_i += 1;
        for i in self.sprites.iter_mut() {
            // i.set_rect(Rect::new(
            //     100. + (self.fps_cnt as f32).sin(),
            //     100.,
            //     150.,
            //     150.,
            // ));
        }
        self.sprites.push(Sprite::new(
            &self.layout,
            100.,
            100.,
            Transform {
                rotation: Vec3::new(0., 0., 0.),
                scale: Vec3::splat(4.),
                translation: Vec3::new(50. - self.last_i as f32 * 50., 50., 0.),
                ..Default::default()
            },
            self.texture.clone(),
            None,
        ));
        self.fps_cnt += 1;
        if self.timer.is_ended() {
            self.timer.reset();
            println!("{}", self.fps_cnt);
            self.fps_cnt = 0;
        }
        if self.input.is_pressed_key(KeyCode::KeyO) {
            self.camera.set_scale(self.camera.get_scale() - 0.1);
        }
        if self.input.is_pressed_key(KeyCode::KeyI) {
            self.camera.set_scale(self.camera.get_scale() + 0.1);
        }
        let mut trans = Vec3::splat(0.);
        if self.input.is_down_key(KeyCode::KeyA) {
            trans.x -= 1.;
        }
        if self.input.is_down_key(KeyCode::KeyD) {
            trans.x += 1.;
        }
        if self.input.is_down_key(KeyCode::KeyW) {
            trans.y += 1.;
        }
        if self.input.is_down_key(KeyCode::KeyS) {
            trans.y -= 1.;
        }
        let mut transform = self.camera.get_transform();
        const SPEED: f32 = 10.;
        transform.translation += trans * SPEED;
        self.camera.set_transform(transform);
        self.input.tick();
    }
    fn window_event(&mut self, event: WindowEvent, context: &mut AppContext, _window: CatWindow) {
        self.input.window_event(event.clone());
        match event {
            WindowEvent::CloseRequested => {
                context.exit();
            }
            _ => {}
        }
    }
    fn render(&mut self, render: &mut cat_render::render::Renderer) {
        render.start_render_for_camera(
            &mut self.camera,
            Some(Color::srgb_255(200., 200., 200.)),
            |render| {
                for sprite in self.sprites.iter_mut() {
                    sprite.render(render);
                }
            },
        );
    }
}
