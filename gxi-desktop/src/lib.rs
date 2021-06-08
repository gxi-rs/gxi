//! # Gxi Desktop Binds
//! *gxi-desktop bindings for the [gxi-rs](https://github.com/gxi-rs) project*
//!
//! This project covers all the widgets standardised in the
//! [GTK3 docs](https://developer.gnome.org/gtk3/stable/gtkobjects.html).

// re-exports
pub use glib;
pub use gtk::prelude::*;
pub use gxi;
pub use tokio;
//
pub use containers::*;
pub use impl_widget::*;
pub use run::*;
pub use widgets::*;

mod containers;
mod run;
mod widgets;
#[macro_use]
mod impl_macros;
mod impl_widget;
