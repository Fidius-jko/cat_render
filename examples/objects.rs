use cat_render::{
    prelude::*,
    render::{
        camera::{Camera2D, Camera2DOptions},
        small::Transform,
    },
    utils::{fs::Filesystem, render::sprite::Sprite},
};
use winit::{
    event::KeyEvent,
    keyboard::{KeyCode, PhysicalKey},
};

fn main() {
    let _ = App::run();
}

pub struct App {
    sprite: Sprite,
    sprite2: Sprite,
    camera: Camera2D,
}

impl CatApp for App {
    fn config() -> AppConfig {
        AppConfig {
            loop_type: LoopType::Active,
        }
    }
    fn new(context: &mut AppContext) -> Self {
        let window =
            context.create_window(WindowAttributes::default().with_title("Objects example"));
        let surface = context.create_surface_for_window(&window).unwrap();
        let mut camera = Camera2D::new(Camera2DOptions {
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            surface: surface.clone(),
            viewport_origin: Vec2::new(0.5, 0.5),
            ..Default::default()
        });
        let texture = context
            .get_mut_renderer()
            .create_texture_from_bytes(&Filesystem::get().read("assets/happy-tree.png").unwrap())
            .unwrap();
        let sprite = Sprite::new(
            context,
            &mut camera,
            100.,
            100.,
            Transform {
                rotation: Vec3::new(0., 0., 0.),
                scale: Vec3::splat(4.),
                translation: Vec3::new(50., 50., 0.),
                ..Default::default()
            },
            texture.clone(),
        );
        let sprite2 = Sprite::new(
            context,
            &mut camera,
            50.,
            50.,
            Transform {
                rotation: Vec3::new(0., 0., 0.),
                scale: Vec3::splat(4.),
                translation: Vec3::new(50., 50., 0.),
                ..Default::default()
            },
            texture,
        );
        Self {
            camera,
            sprite,
            sprite2,
        }
    }
    fn update(&mut self, _context: &mut AppContext) {}
    fn window_event(&mut self, event: WindowEvent, context: &mut AppContext, _window: CatWindow) {
        match event {
            WindowEvent::CloseRequested => {
                context.exit();
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                let KeyEvent {
                    physical_key,
                    logical_key: _,
                    text: _,
                    location: _,
                    state: _,
                    repeat,
                    ..
                } = event;
                if physical_key == PhysicalKey::Code(KeyCode::KeyO) && !repeat {
                    self.camera.set_scale(self.camera.get_scale() - 0.1);
                }
                if physical_key == PhysicalKey::Code(KeyCode::KeyI) && !repeat {
                    self.camera.set_scale(self.camera.get_scale() + 0.1);
                }
                let mut trans = Vec3::splat(0.);
                if physical_key == PhysicalKey::Code(KeyCode::KeyA) {
                    trans.x -= 1.;
                }
                if physical_key == PhysicalKey::Code(KeyCode::KeyD) {
                    trans.x += 1.;
                }
                if physical_key == PhysicalKey::Code(KeyCode::KeyW) {
                    trans.y += 1.;
                }
                if physical_key == PhysicalKey::Code(KeyCode::KeyS) {
                    trans.y -= 1.;
                }
                let mut transform = self.camera.get_transform();
                const SPEED: f32 = 1.;
                transform.translation += trans * SPEED;
                self.camera.set_transform(transform);
            }
            _ => {}
        }
    }
    fn render(&mut self, render: &mut cat_render::render::Renderer) {
        render.start_render_for_camera(
            &mut self.camera,
            Some(Color::srgb_255(200., 200., 200.)),
            |render| {
                self.sprite.render(render);
                self.sprite2.render(render);
            },
        );
    }
}
