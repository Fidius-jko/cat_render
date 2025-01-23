use cat_render::prelude::*;

fn main() {
    let _ = App::run();
}

pub struct App {}

impl CatApp for App {
    type ExitCode = u32;

    fn config() -> AppConfig {
        AppConfig {
            loop_type: LoopType::Active,
        }
    }
    fn new(context: &mut AppContext) -> Self {
        let window = context.create_window(WindowAttributes::default().with_title("Test example"));
        Self {}
    }
    fn update(&mut self, context: &mut AppContext) {}
    fn window_event(&mut self, event: WindowEvent, context: &mut AppContext, _window: CatWindow) {
        match event {
            WindowEvent::CloseRequested => {
                context.exit();
            }
            _ => {}
        }
    }
}
