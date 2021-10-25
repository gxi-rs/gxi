pub use run::*;
pub use top_level::*;
pub use web_container::*;
pub use web_container_wrapper::*;

#[cfg(feature = "async-web")]
pub use wasm_bindgen_futures::*;

mod gen_macros;
mod run;
mod top_level;
mod web_container;
mod web_container_wrapper;

/// Smallest element of the native widget system which can be added to other
/// widgets but it itself may or may not have the ability to hold a child
pub type NativeWidget = web_sys::Node;

/// An element of the native widget system which can hold a child
pub type NativeContainerWidget = WebContainerWrapper;

/// An
pub type Element = WebContainer;
