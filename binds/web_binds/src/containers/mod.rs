pub use a::*;
pub use address::*;
pub use article::*;
pub use aside::*;
pub use body::*;
pub use button::*;
pub use caption::*;
pub use col::*;
pub use colgroup::*;
pub use div::*;
pub use form::*;
pub use head::*;
pub use table::*;
pub use text::*;
pub use footer::*;

mod body;
mod button;
mod div;
#[macro_use]
mod container_impl_macro;
mod a;
mod head;
mod table;
mod text;
mod form;
mod caption;
mod col;
mod colgroup;
mod address;
mod article;
mod aside;
mod footer;
