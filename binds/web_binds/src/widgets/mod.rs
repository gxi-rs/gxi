pub use img::*;
pub use link::*;
pub use meta::*;
pub use script::*;
pub use style::*;
pub use title::*;
pub use hr::*;

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
