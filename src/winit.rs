use std::time::Duration;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::WindowId,
};

use crate::{
    app::CatApp,
    context::{AppContext, StaticContext},
    utils::timer::Timer,
    window::CatWindow,
};

pub(crate) fn run<App: CatApp>() {
    let mut app: WinitApp<App> = WinitApp::default();
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut app).unwrap();
}

pub struct WinitContext<'a> {
    pub event_loop: &'a ActiveEventLoop,
}

pub(crate) struct WinitApp<A: CatApp> {
    app: Option<A>,
    context: Option<StaticContext>,
    fps_timer: Option<Timer>,
}
impl<App: CatApp> Default for WinitApp<App> {
    fn default() -> Self {
        Self {
            app: Default::default(),
            context: Default::default(),
            fps_timer: None,
        }
    }
}
impl<App: CatApp> WinitApp<App> {}
impl<App: CatApp> ApplicationHandler for WinitApp<App> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut context = StaticContext::new();
        let mut app_context = AppContext::new(WinitContext { event_loop }, &mut context);
        self.app = Some(App::new(&mut app_context));
        self.context = Some(context);

        if self.context.as_ref().unwrap().fps != 0 {
            self.fps_timer = Some(Timer::new(Duration::from_secs_f32(
                1. / self.context.as_ref().unwrap().fps as f32,
            )));
        } else {
            self.fps_timer = Some(Timer::new(Duration::from_secs_f32(0.)));
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let mut app_context =
            AppContext::new(WinitContext { event_loop }, self.context.as_mut().unwrap());
        match event {
            WindowEvent::RedrawRequested => {
                app_context.base.windows.request_redraw();
                if self.fps_timer.as_ref().unwrap().is_ended() {
                    let fps = app_context.base.fps;
                    self.app.as_mut().unwrap().update(&mut app_context);

                    self.app
                        .as_mut()
                        .unwrap()
                        .render(&mut app_context.base.renderer);
                    if app_context.base.renderer.needs_exit {
                        app_context.exit = true;
                    }

                    if fps != app_context.base.fps {
                        if app_context.base.fps != 0 {
                            self.fps_timer = Some(Timer::new(Duration::from_secs_f32(
                                1. / app_context.base.fps as f32,
                            )));
                        } else {
                            self.fps_timer = Some(Timer::new(Duration::from_secs_f32(0.)));
                        }
                    }
                    self.fps_timer.as_mut().unwrap().reset();
                }
            }
            WindowEvent::Resized(physical_size) => {
                app_context.base.renderer.on_resize(&id, physical_size);
            }
            _ => {}
        }

        self.app
            .as_mut()
            .unwrap()
            .window_event(event, &mut app_context, CatWindow { id });

        if app_context.exit {
            app_context.winit_context.event_loop.exit();
        }
    }
}
