pub(crate) mod winit;

pub mod app;
pub mod context;
pub mod render;
pub mod utils;
pub mod window;

pub mod prelude {
    pub use crate::app::*;
    pub use crate::context::AppContext;
    pub use crate::render::surface::SurfaceId;
    pub use crate::render::Color;
    pub use crate::window::*;
    pub use glam::*;
}
