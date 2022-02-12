//! # Context Trees
//!
//! This tree structure is a unique way of representing one way binding between
//! `state`, `readers` and `writers` in an observable event based pattern.
//!
//! > This method can introduce `memory leaks` and `undefined behaviour`, if not used
//! > correctly. Therefore it is recommended to stick with the `gxi` macro to produce
//! > bug free code.
//!
//! A context node is defined a as a memory pool of nodes and sub trees. Each type of
//! context node come with their own side effect, ment to be used in a particular situation
//! only.
//!
//! ## Lifecycle
//!
//! ```md
//!    {
//!
//!       [context node]
//!
//!       [state]
//!          |
//!          |
//!          |  [observable node]
//!          |        /
//!          |        |
//!           ----- (view)
//!
//!    }
//! ```

mod const_context;
mod indexed_context;
mod mem_dump;
mod vnode_context;

pub use const_context::*;
pub use indexed_context::*;
pub use mem_dump::*;
pub use vnode_context::*;
