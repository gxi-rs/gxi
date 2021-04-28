//! # GXI
//! *Cross-Platform Native Widget based Component System in Rust*
//!
//! This project implements a component system of GUI widgets and nodes. Using rust [proc-macros](https://doc.rust-lang.org/reference/procedural-macros.html) compiles the component tree to optimized logical n-binary tree flow which `prevents` the use of any [virtual dom](https://reactjs.org/docs/faq-internals.html) or [diffing algorithms](https://reactjs.org/docs/reconciliation.html). Making the component system `zero cost`. Hence the components are` highly optimized`, `performant`, and `customized` to meet the needs of each project while maintaining the standard features of frameworks like `React`. Built-in `async support` allows for quick and performant abstractions to rust futures.
//!
//! Since the framework is a compiler, therefore, it allows mixing of platform dependent and independent components, i.e the framework provides components like `div`, `h1` (platform dependent) and [React Native](https://reactnative.dev/) like platform-independent components like `Text` and `View`. Therefore making the code portable without losing deep control of the native system.
//!
//! Read more [here](https://github.com/aniketfuryrocks/gxi)

#[cfg(feature = "desktop")]
pub use gxi_desktop_binds::*;
pub use gxi_interface::*;
#[cfg(feature = "web")]
pub use gxi_web_binds::*;
