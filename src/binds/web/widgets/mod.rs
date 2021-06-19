pub use base::*;
pub use hr::*;
pub use iframe::*;
pub use img::*;
pub use input::*;
pub use link::*;
pub use meta::*;
pub use script::*;
pub use style::*;
pub use title::*;

mod img;
#[macro_use]
mod widget_impl_macro;
mod base;
mod hr;
mod iframe;
mod input;
mod link;
mod meta;
mod script;
mod style;
mod title;
