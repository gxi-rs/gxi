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
pub mod observer_builder;
pub mod state;
pub mod sub_tree;
#[macro_use]
pub mod optional_parse;
pub mod blocks;
pub mod lifetime;
pub mod observables;
pub mod snippets;

pub use crate::blocks::RootBlock;
