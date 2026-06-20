use std::time::Duration;

use cat_render::{
    prelude::*,
    render::{
        camera::{Camera, Camera2D, Camera2DOptions},
        small::Transform,
        texture::Texture,
        Render,
    },
    utils::{
        fs::Filesystem,
        input::Input,
        render::sprite::{Sprite, SpriteLayout},
        text::{default_max, line_height, Font},
        timer::Timer,
        to_radians,
    },
};
use cosmic_text::Attrs;
use winit::{
    dpi::{PhysicalPosition, Position},
    keyboard::KeyCode,
};
fn main() {
    let _ = App::run();
}

pub struct App {
    camera: Camera2D,
    ui: Ui,
    input: Input,
    sprite: Sprite,
    tick: u32,
    timer: Timer,
    frames: u32,
    fps: u32,
}

impl CatApp for App {
    fn config() -> AppConfig {
        AppConfig {
            loop_type: LoopType::Active,
        }
    }
    fn new(context: &mut AppContext) -> Self {
        context.set_fps(60);
        cat_render::utils::init_utils(context);
        let window = context.create_window(
            WindowAttributes::default()
                .with_title("Objects example")
                .with_position(Position::Physical(PhysicalPosition::new(1000, 300))),
        );
        let surface = context.create_surface_for_window(&window).unwrap();
        let camera = Camera2D::new(Camera2DOptions {
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

        let sprite_layout = SpriteLayout::new(context, camera.get_bind_group(), None);
        let sprite = Sprite::new(
            &sprite_layout,
            100.,
            100.,
            Transform {
                rotation: Vec3::new(0., 0., 0.),
                scale: Vec3::splat(2.),
                translation: Vec3::new(0., 0., 0.),
                ..Default::default()
            },
            texture.clone(),
            None,
        );

        Self {
            camera,
            ui: Ui::new(context, surface.clone(), "asdasd"),
            input: Input::new(),
            sprite,
            tick: 0,
            timer: Timer::new(Duration::from_secs(1)),
            frames: 0,
            fps: 0,
        }
    }
    fn update(&mut self, _context: &mut AppContext, delta: f32) {
        self.frames += 1;
        if self.timer.is_ended() {
            self.fps = self.frames;
            self.frames = 0;
            println!("{}", self.fps);
            self.timer.reset();
        }
        self.tick += 1;
        self.ui
            .update_text(format!("Оу уже прошло: {} тиков! {}asdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sdasdjlkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkasddddddddddd\nasdasdasd\nasdasd\nasdasd\nasda sd", self.tick, self.fps).as_str());

        let mut transform = self.sprite.get_transform();
        transform.rotation.z += to_radians(2.);
        transform.scale.x = ((self.tick as f32 / 30.).sin() + 1.1) * 2.;
        transform.scale.y = ((self.tick as f32 / 30.).cos() + 1.1) * 2.;
        self.sprite.update_transform(transform);

        if self.input.is_pressed_key(KeyCode::KeyO) {
            self.camera.set_scale(self.camera.get_scale() * 0.9);
        }
        if self.input.is_pressed_key(KeyCode::KeyI) {
            self.camera.set_scale(self.camera.get_scale() * 1.1);
        }

        let mut trans = Vec3::splat(0.);

        if self.input.is_down_key(KeyCode::KeyW) {
            trans.y += 1.;
        }
        if self.input.is_down_key(KeyCode::KeyS) {
            trans.y -= 1.;
        }
        if self.input.is_down_key(KeyCode::KeyA) {
            trans.x -= 1.;
        }
        if self.input.is_down_key(KeyCode::KeyD) {
            trans.x += 1.;
        }

        const SPEED: f32 = 10.;
        let mut transform = self.camera.get_transform();
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
    fn render(&mut self, renderer: &mut cat_render::render::Renderer) {
        renderer.start_render_for_surface(
            self.camera.get_surface_id(),
            Some(Color::srgb_255(155., 155., 155.)),
            None,
            |render| {
                render.set_camera(&mut self.camera);
                self.sprite.render(render);
                self.ui.render(render);
            },
        );
    }
}

const FONT_SIZE: f32 = 40.0;
pub struct Ui {
    ui_camera: Camera2D,
    font: Font,
    text_layout: SpriteLayout,
    text: Sprite,
}
impl Ui {
    pub fn render(&mut self, render: &mut Render) {
        render.set_camera(&mut self.ui_camera);
        self.text.render(render);
    }
    pub fn update_text(&mut self, text: &str) {
        self.text.update_texture(self.font.render_to_new_texture(
            text,
            FONT_SIZE,
            line_height(FONT_SIZE, 0.4 * FONT_SIZE),
            default_max(),
            Color::srgb_255(0., 0., 0.),
            Attrs::new(),
        ));
    }
    pub fn new(context: &mut AppContext, surface: SurfaceId, text: &str) -> Self {
        let ui_camera = Camera2D::new(Camera2DOptions {
            surface: surface.clone(),
            viewport_origin: Vec2::new(0., 1.),
            ..Default::default()
        });

        let mut font = Font::new(cosmic_text::fontdb::Source::File(
            "assets/FiraMono-Medium.ttf".into(),
        ));
        let text_layout =
            SpriteLayout::new(context, ui_camera.get_bind_group(), Some(Vec2::splat(0.)));

        let sprite = Sprite::new(
            &text_layout,
            0.,
            0.,
            Transform {
                rotation: Vec3::new(0., 0., 0.),
                scale: Vec3::splat(1.),
                translation: Vec3::new(0., 0., 0.),
                ..Default::default()
            },
            font.render_to_new_texture(
                text,
                FONT_SIZE,
                line_height(FONT_SIZE, 0.4 * FONT_SIZE),
                default_max(),
                Color::srgb_255(0., 0., 0.),
                Attrs::new(),
            ),
            None,
        );
        Self {
            ui_camera,
            font,
            text_layout,
            text: sprite,
        }
    }
}
