//! ## Observable Pattern
//!
//! The observable system is nothing more than a wrapper around, vectors of
//! observable callback closures. The list of `observers` can be notified about
//! a state change using an associated `notify()` function.
//!
//! `&RefCell<V>` is passed to the `observer` to prevent multiple mutable borrows.

mod async_observable;
mod async_state;
mod observable;
mod observers;
#[allow(clippy::module_inception)]
mod state;

pub use async_observable::*;
pub use async_state::*;
pub use observable::*;
pub use observers::*;
pub use state::*;
