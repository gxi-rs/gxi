//! ## Aim
//!
//! 1. Case Sensitive
//! 2. Self Documenting
//! 3. Similar to `rust` syntax
//! 4. Serializable
//!
//! ## Syntax
//!
//! Refer to [`RootBlock`](struct@RootBlock)
mod observer_builder;
mod scope;
mod sub_tree;
#[macro_use]
mod optional_parse;
mod blocks;
mod lifetime;

pub use blocks::*;
pub use lifetime::*;
pub use observer_builder::*;
pub use optional_parse::*;
pub use scope::*;
pub use sub_tree::*;
