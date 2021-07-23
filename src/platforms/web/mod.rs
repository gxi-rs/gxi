pub use run::*;
pub use top_level::*;
pub use web_container::*;
pub use web_container_wrapper::*;

mod run;
mod top_level;
mod web_container;
mod web_container_wrapper;

/// Smallest element of the native widget system which can be added to other
/// widgets but it itself may or may not have the ability to hold a child
pub type NativeWidget = web_sys::Node;

/// An element of the native widget system which can hold a child
pub type NativeContainer = WebContainerWrapper;
