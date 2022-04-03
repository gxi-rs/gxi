//! Each expression following a different role and syntax
//! inside the gxi macro is termed as a block.
//!
//! As the name suggests the [`RootBlock`](struct@RootBlock) lays
//! at the root of the gxi macro.
mod conditional;
mod execution;
mod node;
mod root;

pub use conditional::*;
pub use execution::*;
pub use node::*;
pub use root::*;
