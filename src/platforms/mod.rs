#[cfg(not(any(feature = "web", feature = "desktop")))]
pub use default::*;
#[cfg(feature = "desktop")]
pub use desktop::*;
#[cfg(feature = "web")]
pub use web::*;

#[cfg(feature = "desktop")]
mod desktop;
#[cfg(feature = "web")]
mod web;

#[cfg(not(any(feature = "web", feature = "desktop")))]
mod default;
