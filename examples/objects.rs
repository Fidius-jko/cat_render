use cat_render::{
    prelude::*,
    render::{
        camera::{Camera2D, Camera2DOptions},
        small::Transform,
        texture::Texture,
    },
    utils::{fs::Filesystem, input::Input, render::sprite::Sprite},
};
use winit::keyboard::KeyCode;
fn main() {
    let _ = App::run();
}

pub struct App {
    sprite: Sprite,
    camera: Camera2D,
    input: Input,
}

impl CatApp for App {
    fn config() -> AppConfig {
        AppConfig {
            loop_type: LoopType::Active,
        }
    }
    fn new(context: &mut AppContext) -> Self {
        cat_render::utils::init_utils(context);
        let window =
            context.create_window(WindowAttributes::default().with_title("Objects example"));
        let surface = context.create_surface_for_window(&window).unwrap();
        let mut camera = Camera2D::new(Camera2DOptions {
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            surface: surface.clone(),
            viewport_origin: Vec2::new(0.5, 0.5),
            ..Default::default()
        });

        let texture = Texture::from_bytes(
            &Filesystem::get().read("assets/happy-tree.png").unwrap(),
            wgpu::FilterMode::Nearest,
        )
        .unwrap();

        let sprite = Sprite::new(
            context,
            &mut camera,
            100.,
            100.,
            Transform {
                rotation: Vec3::new(0., 0., 0.),
                scale: Vec3::splat(4.),
                translation: Vec3::new(0., 0., 0.),
                ..Default::default()
            },
            texture.clone(),
        );
        println!("A");

        Self {
            camera,
            sprite,
            input: Input::new(),
        }
    }
    fn update(&mut self, _context: &mut AppContext, _delta: f32) {
        if self.input.is_pressed_key(KeyCode::KeyO) {
            self.camera.set_scale(self.camera.get_scale() - 0.1);
        }
        if self.input.is_pressed_key(KeyCode::KeyI) {
            self.camera.set_scale(self.camera.get_scale() + 0.1);
        }
        if self.input.is_pressed_key(KeyCode::KeyR) {
            let mut transf = self.sprite.get_transform();
            transf.rotation.z += 1. / 10.;
            self.sprite.update_transform(transf);
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
        // let mut transform = self.sprite.get_transform();
        const SPEED: f32 = 10.;
        transform.translation -= trans * SPEED;
        // self.sprite.update_transform(transform);
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
            Some(Color::srgb_255(100., 100., 100.)),
            |render| {
                self.sprite.render(render);
            },
        );
    }
}
