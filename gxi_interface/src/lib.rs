#[cfg(feature = "desktop")]
pub use gtk;
#[cfg(feature = "web")]
pub use wasm_bindgen;
#[cfg(feature = "web")]
pub use web_sys;

pub use c::c;
pub use comp::comp;
pub use nodes::*;
pub use parsers::{comp_new, comp_state};
pub use should_render::*;
pub use update::update;

mod nodes;
mod should_render;

pub type AsyncResult<T> = Result<T, Box<dyn std::error::Error>>;
