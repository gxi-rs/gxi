//! # GXI
//! *Cross-Platform Native Widget based Component System in Rust*
//!
//! This project implements a component system of GUI widgets and nodes. Using rust [proc-macros](https://doc.rust-lang.org/reference/procedural-macros.html) compiles the component tree to optimized logical n-binary tree flow which `prevents` the use of any [virtual dom](https://reactjs.org/docs/faq-internals.html) or [diffing algorithms](https://reactjs.org/docs/reconciliation.html). Making the component system `zero cost`. Hence the components are` highly optimized`, `performant`, and `customized` to meet the needs of each project while maintaining the standard features of frameworks like `React`. Built-in `async support` allows for quick and performant abstractions to rust futures.
//!
//! Since the framework is a compiler, therefore, it allows mixing of platform dependent and independent components, i.e the framework provides components like `div`, `h1` (platform dependent) and [React Native](https://reactnative.dev/) like platform-independent components like `Text` and `View`. Therefore making the code portable without losing deep control of the native system.
//!
//! Read more [here](https://github.com/aniketfuryrocks/gxi)

#[cfg(all(feature = "web", feature = "desktop"))]
compile_error!("Cannot enable both `web` and `desktop` features.");

#[cfg(not(any(feature = "web", feature = "desktop")))]
compile_error!("Either `web` or `desktop` feature must be enabled.");

macro_rules! transparent_block {( $($tt:tt)* ) => ( $($tt)* )}

#[cfg(any(feature = "web", feature = "desktop"))]
#[cfg(not(all(feature = "web", feature = "desktop")))]
transparent_block! {
    mod parser_macros;
    mod binds;
    mod should_render;
    mod nodes;
    pub use binds::*;
    pub use parser_macros::*;
    pub use gxi_macro::gxi;
    pub use nodes::*;
    pub use should_render::*;

    pub type AsyncResult<T> = Result<T, Box<dyn std::error::Error>>;
}

