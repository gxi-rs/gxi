#[cfg(feature = "desktop")]
pub use gtk;
#[cfg(feature = "web")]
pub use wasm_bindgen;
#[cfg(feature = "web")]
pub use web_sys;

pub use c::c;
pub use comp::comp;
pub use nodes::*;
pub use parsers::comp_init;
pub use update::update;

mod nodes;

pub type AsyncResult<T> = Result<T, Box<dyn std::error::Error>>;
