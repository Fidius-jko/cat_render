// TODO: Change Hashmap to faster from hashbrown
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::{
    app::LoopType,
    render::{surface::SurfaceId, Renderer},
    window::{CatWindow, WindowAttributes, Windows},
    winit::WinitContext,
};

pub(crate) struct StaticContext {
    pub windows: Windows,
    pub renderer: Renderer,
    pub fps: u32,
    pub resources: Resources,
}

impl StaticContext {
    pub fn new() -> Self {
        Self {
            fps: 0,
            windows: Windows::new(),
            renderer: pollster::block_on(Renderer::new()),
            resources: Resources::new(),
        }
    }
}

pub(crate) struct Resources {
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }
    pub fn insert<R: Any>(&mut self, res: R) {
        self.resources.insert(TypeId::of::<R>(), Box::new(res));
    }
    pub fn contains<R: Any>(&self) -> bool {
        self.resources.contains_key(&TypeId::of::<R>())
    }

    pub fn get<R: Any>(&self) -> Option<&R> {
        match self.resources.get(&TypeId::of::<R>()) {
            Some(r) => r.downcast_ref(),
            None => None,
        }
    }
    pub fn get_mut<R: Any>(&mut self) -> Option<&mut R> {
        match self.resources.get_mut(&TypeId::of::<R>()) {
            Some(r) => r.downcast_mut(),
            None => None,
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
    //-----------------------------RENDERER---------------------------//
    pub fn get_mut_renderer(&mut self) -> &mut Renderer {
        &mut self.base.renderer
    }
    pub fn get_renderer(&self) -> &Renderer {
        &self.base.renderer
    }
    //------RENDERING-----/
    pub fn create_surface_for_window(&mut self, window: &CatWindow) -> Option<SurfaceId> {
        Some(
            self.base
                .renderer
                .create_surface(self.base.windows.get(window)?),
        )
    }
    //-----------------------------RESOURCES---------------------------//
    pub fn insert_resource<R: Any>(&mut self, res: R) {
        self.base.resources.insert(res);
    }
    pub fn contains_resource<R: Any>(&self) -> bool {
        self.base.resources.contains::<R>()
    }

    pub fn get_resource<R: Any>(&self) -> Option<&R> {
        self.base.resources.get::<R>()
    }
    pub fn get_mut_resource<R: Any>(&mut self) -> Option<&mut R> {
        self.base.resources.get_mut::<R>()
    }
}
