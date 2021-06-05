//! # Gxi Web Binds
//! *gxi-web bindings for the [gxi-rs](https://github.com/gxi-rs) project*
//!
//! This project covers all the elements standardised in the
//! [MDN docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element)
//! with all the standard attributes.

pub use containers::*;
pub use global_attributes::*;
pub use run::*;
pub use wasm_bindgen::*;
pub use wasm_bindgen::prelude::*;
pub use wasm_bindgen_futures::*;
pub use widgets::*;

mod containers;
mod run;
#[macro_use]
pub mod util_macros;
#[macro_use]
mod web_impl_macros;
mod global_attributes;
mod widgets;
