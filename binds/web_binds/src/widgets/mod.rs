pub use img::*;
pub use link::*;
pub use meta::*;
pub use script::*;
pub use style::*;
pub use title::*;
pub use hr::*;
pub use iframe::*;
pub use base::*;

mod img;
#[macro_use]
mod widget_impl_macro;
mod link;
mod meta;
mod script;
mod style;
mod title;
mod input;
mod hr;
mod iframe;
mod base;
