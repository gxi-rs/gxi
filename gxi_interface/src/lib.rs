#[cfg(feature = "desktop")]
pub use gtk;
#[cfg(feature = "web")]
pub use wasm_bindgen;
#[cfg(feature = "web")]
pub use web_sys;

pub use gxi_macro::gxi;
pub use gxi_parsers::{comp_new, comp_state};
pub use gxi_update_macro::update;
pub use nodes::*;
pub use should_render::*;

mod nodes;
mod should_render;

pub type AsyncResult<T> = Result<T, Box<dyn std::error::Error>>;
