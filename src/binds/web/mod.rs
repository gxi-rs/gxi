//! # Gxi Web Binds
//! *gxi-web bindings for the [gxi-rs](https://github.com/gxi-rs) project*
//!
//! This project covers all the elements standardised in the
//! [MDN docs](https://developer.mozilla.org/en-US/docs/Web/HTML/Element)
//! with all the standard attributes.

pub use wasm_bindgen_futures;
pub use web_sys;
pub use wasm_bindgen;
pub use wasm_bindgen::prelude::*; // required

pub use global_attributes::*; // required
pub use containers::*;
pub use top_level_widgets::*;
pub use widgets::*;
pub use run::*;

mod containers;
mod run;
#[macro_use]
pub mod util_macros;
#[macro_use]
mod web_impl_macros;
mod global_attributes;
mod top_level_widgets;
mod widgets;

pub type NativeWidget = web_sys::Node;
pub type NativeContainer = web_sys::Element;
