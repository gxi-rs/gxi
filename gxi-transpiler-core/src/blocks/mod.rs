//! Each expression following a different role and syntax
//! inside the gxi macro is termed as a block.
//!
//! As the name suggests the [`RootBlock`](struct@RootBlock) lays
//! at the root of the gxi macro.
pub mod conditional;
pub mod execution;
pub mod node;
mod root;
pub use root::*;
