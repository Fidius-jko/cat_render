use std::{collections::HashMap, sync::Arc};
use winit::window::{Window, WindowId};

use crate::winit::WinitContext;

pub type WindowAttributes = winit::window::WindowAttributes;
pub type WindowEvent = winit::event::WindowEvent;

// use super::Surfaces;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CatWindow {
    pub(crate) id: WindowId,
}

pub(crate) struct Windows {
    windows: HashMap<WindowId, Arc<Window>>,
}

impl Windows {
    pub(crate) fn request_redraw(&self) {
        match self.windows.iter().next() {
            Some(window) => {
                window.1.request_redraw();
            }
            None => {}
        }
    }
    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

    pub fn delete(&mut self, window: CatWindow) {
        self.windows.remove(&window.id);
        // let mut surfaces = Surfaces::get();
        // let surface = surfaces.get_surface_id_from_window(&window.id);
        // match surface {
        //     Some(s) => surfaces.delete_surface(s),
        //     None => {}
        // }
    }
    pub fn exists(&self, window: &CatWindow) -> bool {
        self.windows.contains_key(&(window.id))
    }
    pub fn create(
        &mut self,
        winit: &WinitContext,
        window_attributes: WindowAttributes,
    ) -> CatWindow {
        let new = winit.event_loop.create_window(window_attributes).unwrap();
        let id = new.id();
        self.windows.insert(id, Arc::new(new));
        return CatWindow { id };
    }
    // pub fn get(&self, window: &CatWindow) -> Option<Arc<Window>> {
    //     self.windows.get(&window.id).cloned()
    // }
}
