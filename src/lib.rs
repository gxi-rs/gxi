//! # GXI
//! *Cross-Platform Native Widget based Component System in Rust*
//!
//! This project implements a component system of GUI widgets and nodes. Using rust [proc-macros](https://doc.rust-lang.org/reference/procedural-macros.html) compiles the component tree to optimized logical n-binary tree flow which `prevents` the use of any [virtual dom](https://reactjs.org/docs/faq-internals.html) or [diffing algorithms](https://reactjs.org/docs/reconciliation.html). Making the component system `zero cost`. Hence the components are` highly optimized`, `performant`, and `customized` to meet the needs of each project while maintaining the standard features of frameworks like `React`. Built-in `async support` allows for quick and performant abstractions to rust futures.
//!
//! Since the framework is a compiler, therefore, it allows mixing of platform dependent and independent components, i.e the framework provides components like `div`, `h1` (platform dependent) and [React Native](https://reactnative.dev/) like platform-independent components like `Text` and `View`. Therefore making the code portable without losing deep control of the native system.
//!
//! Read more [here](https://github.com/aniketfuryrocks/gxi)

pub use binds::*;
pub use components::*;
pub use gxi_macro::gxi;
pub use interface::*;
pub use parser_macros::*;
pub use should_render::*;

mod components;
mod interface;
mod parser_macros;
mod should_render;
#[macro_use]
mod node_impl_macros;
mod binds;

pub type AsyncResult<T> = Result<T, Box<dyn std::error::Error>>;
