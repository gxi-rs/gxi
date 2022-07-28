//! # Context Trees
//!
//! This tree structure is a unique way of representing one way binding between
//! `state`, `readers` and `writers` in an observable event based pattern.
//!
//! > Note:
//! >
//! > This method can introduce `memory leaks` and `undefined behavior`, if not used
//! > correctly. Therefore, it is recommended to stick with the `gxi` macro to produce
//! > bug free code.
//!
//! A context node is defined as a memory pool of nodes and sub trees. Each type of
//! context node come with their own side effect, meant to be used in a particular situation.
//!
//! ## Lifecycle
//!
//! ```md
//!    {
//!
//!     [context node]
//!        |
//!        |    [ state ] ----
//!        |       |          |
//!        |       | (owns)   |
//!        |       |          |
//!        |    [writer]      |  Weakly owns state (Exists till state exists),
//!        |       |          |  by binding weak reference of itself into state's observer
//!        |       |          |  which intern exist till writer does
//!        |       |          |
//!        ----------------(reader)
//!     
//!        context owns writer and reader.
//!    }
//! ```
//!
//! ## Concept
//!
//! `writers` *strongly* own state. Until and unless there is at-least one node
//! which mutates state, readers need to exist.
//!
//! `readers` bind an `observer` to `state` by weekly moving themselves into the state.
//!
//!  in-case `reader` is dynamic (Conditional, Iterative), local context is moved to global context
//!  and a weak reference of local context is moved to state observer.
//!
//!  if `reader` drops before the `state`, binded observer is romoved.
//!  if all `writers` drop, state is removed. but, readers exist till the lifetime of local
//!  context.
//!
//! `context` own all `nodes` with extended Lifetime (`readers` and `writers`).
//!
//!
//! Each function returns their context, which is moved into a master context.
//! The master context lives throughout the lifetime of the application.
//!
//! ## Dropping
//!
//! `state`, `readers` can only be freed when `writers` are dropped.
//!
//! There are basically 2 ways to drop `writers`:
//! 1. Conditional rendering [`IndexedContext`]
//! 2. Iterative rendering (WIP)

mod const_context;
mod indexed_context;
mod mem_dump;
mod vnode_context;

pub use const_context::*;
pub use indexed_context::*;
pub use mem_dump::*;
pub use vnode_context::*;
