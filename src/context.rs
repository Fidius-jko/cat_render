use crate::{
    app::LoopType,
    window::{CatWindow, WindowAttributes, Windows},
    winit::WinitContext,
};

pub(crate) struct StaticContext {
    pub windows: Windows,
    pub fps: u32,
}

impl StaticContext {
    pub fn new() -> Self {
        Self {
            fps: 0,
            windows: Windows::new(),
        }
    }
}

pub struct AppContext<'a> {
    pub(crate) base: &'a mut StaticContext,
    pub(crate) winit_context: WinitContext<'a>,
    pub(crate) exit: bool,
}

impl<'a> AppContext<'a> {
    pub(crate) fn new(winit_context: WinitContext<'a>, base: &'a mut StaticContext) -> Self {
        Self {
            base,
            winit_context,
            exit: false,
        }
    }
    //------------------------------------------------------USER------------------------------------------//
    pub fn set_fps(&mut self, fps: u32) {
        self.base.fps = fps;
    }
    pub fn exit(&mut self) {
        self.exit = true;
    }
    //-----------------------------WINDOWS---------------------------//
    pub fn change_loop_type(&mut self, loop_type: LoopType) {
        use winit::event_loop::ControlFlow;
        let control_flow = match loop_type {
            LoopType::Active => ControlFlow::Poll,
            LoopType::Waiting => ControlFlow::Wait,
        };

        self.winit_context.event_loop.set_control_flow(control_flow);
    }
    pub fn create_window(&mut self, attrs: WindowAttributes) -> CatWindow {
        self.base.windows.create(&self.winit_context, attrs)
    }
    pub fn destroy_window(&mut self, window: CatWindow) {
        self.base.windows.delete(window);
    }
    pub fn exists_window(&self, window: &CatWindow) -> bool {
        self.base.windows.exists(window)
    }
}
