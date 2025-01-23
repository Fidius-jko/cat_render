use anyhow::{Ok, Result};

use crate::{context::AppContext, window::{CatWindow, WindowEvent}};

pub trait CatApp {
    type ExitCode: Default;

    fn config() -> AppConfig;

    fn update(&mut self, context: &mut AppContext);
    fn new(context: &mut AppContext) -> Self;
    fn window_event(&mut self, event: WindowEvent, context: &mut AppContext, window: CatWindow);
}

pub trait AppExt: CatApp {
    fn run() -> Result<Self::ExitCode>;
}

impl<A: CatApp> AppExt for A {
    fn run() -> Result<Self::ExitCode> {
        crate::utils::logger::init_logger();
        crate::winit::run::<A>();
        Ok(Self::ExitCode::default())
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