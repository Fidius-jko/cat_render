use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, Mutex, MutexGuard},
};

use wgpu::{Surface, SurfaceConfiguration};
use winit::{
    dpi::PhysicalSize,
    window::{Window, WindowId},
};

use super::Renderer;

pub(crate) struct Surfaces<'a> {
    surfaces: HashMap<SurfaceId, CatSurface<'a>>,
    window_surfaces: HashMap<WindowId, SurfaceId>,
    last_id: u32,
}

pub struct CatSurface<'a> {
    pub(crate) wgpu_surface: Arc<Surface<'a>>,
    pub(crate) size: PhysicalSize<u32>,
    pub(crate) config: SurfaceConfiguration,
}

impl<'a> Surfaces<'a> {
    pub(crate) fn exists(&self, id: SurfaceId) -> bool {
        self.surfaces.contains_key(&id)
    }
    pub(crate) fn new() -> Self {
        Self {
            surfaces: HashMap::new(),
            last_id: 0,
            window_surfaces: HashMap::new(),
        }
    }
    pub(crate) fn get_surface_id_from_window(&mut self, window: &WindowId) -> Option<SurfaceId> {
        self.window_surfaces.get(window).cloned()
    }
    pub(crate) fn delete_surface(&mut self, surface: SurfaceId) {
        self.surfaces.remove(&surface);
        let mut k = None;
        for (k2, v) in self.window_surfaces.iter() {
            if surface == v.clone() {
                k = Some(k2.clone());
                break;
            }
        }
        match k {
            Some(id) => {
                self.window_surfaces.remove(&id);
            }
            _ => {}
        }
    }

    pub(crate) fn get_surface(&self, id: SurfaceId) -> &CatSurface<'a> {
        self.surfaces.get(&id).unwrap()
    }
    pub(crate) fn get_mut_surface(&mut self, id: SurfaceId) -> &mut CatSurface<'a> {
        self.surfaces.get_mut(&id).unwrap()
    }
    pub(crate) fn create_surface(
        &mut self,
        renderer: &mut Renderer,
        window: Arc<Window>,
    ) -> SurfaceId {
        let surface = renderer
            .instance
            .create_surface(window.clone())
            .expect("Failed to create surface");
        if !renderer.adapter.is_surface_supported(&surface) {
            // WebGl2 requirement TODO
            todo!()
        }
        let size = window.inner_size();

        let surface_caps = surface.get_capabilities(&renderer.adapter);
        // sRGB
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        self.window_surfaces
            .insert(window.id(), SurfaceId(self.last_id));
        self.surfaces.insert(
            SurfaceId(self.last_id),
            CatSurface {
                wgpu_surface: Arc::new(surface),
                size,
                config,
            },
        );
        self.last_id += 1;
        return SurfaceId(self.last_id - 1);
    }
    pub(crate) fn get() -> MutexGuard<'a, Surfaces<'static>> {
        SURFACES.lock().unwrap()
    }
    pub(crate) fn resize_window_surface(
        &mut self,
        renderer: &mut Renderer,
        window: &WindowId,
        new_size: PhysicalSize<u32>,
    ) {
        match self.window_surfaces.get(window) {
            Some(surface) => {
                if new_size.width != 0 && new_size.height != 0 {
                    let surface = self.get_mut_surface(surface.clone());
                    surface.size = new_size;
                    surface.config.width = new_size.width;
                    surface.config.height = new_size.height;
                    surface
                        .wgpu_surface
                        .configure(&renderer.device, &surface.config);
                }
            }
            None => {}
        }
    }
}

pub static SURFACES: LazyLock<Arc<Mutex<Surfaces>>> =
    std::sync::LazyLock::new(|| Arc::new(Mutex::new(Surfaces::new())));

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct SurfaceId(u32);
