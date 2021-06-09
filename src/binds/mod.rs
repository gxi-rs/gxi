#[cfg(feature = "desktop")]
pub use desktop_binds::*;
#[cfg(feature = "web")]
pub use web_binds::*;

#[cfg(feature = "desktop")]
mod desktop_binds;
#[cfg(feature = "web")]
mod web_binds;