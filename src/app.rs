use crate::{
    context::AppContext,
    render::Renderer,
    window::{CatWindow, WindowEvent},
};

pub trait CatApp {
    fn config() -> AppConfig;

    fn update(&mut self, context: &mut AppContext);
    fn new(context: &mut AppContext) -> Self;
    fn window_event(&mut self, event: WindowEvent, context: &mut AppContext, window: CatWindow);
    fn render(&mut self, render: &mut Renderer);
}

pub trait AppExt: CatApp {
    fn run();
}

impl<A: CatApp> AppExt for A {
    fn run() {
        crate::utils::logger::init_logger();
        crate::winit::run::<A>();
    }
}

#[derive(Default)]
pub struct AppConfig {
    pub loop_type: LoopType,
}

#[derive(Default)]
pub enum LoopType {
    #[default]
    Active,
    Waiting,
}
