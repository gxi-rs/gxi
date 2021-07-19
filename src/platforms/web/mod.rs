pub use run::*;
pub use web_container_wrapper::*;
pub use web_container::*;
pub use top_level::*;

mod top_level;
mod run;
mod web_container_wrapper;
mod web_container;

/// Smallest element of the native widget system which can be added to other
/// widgets but it itself may or may not have the ability to hold a child
pub type NativeWidget = web_sys::Node;

/// An element of the native widget system which can hold a child
pub type NativeContainer = WebContainerWrapper;

