//! # Gxi Desktop Binds
//! *desktop bindings for the [gxi-rs](https://github.com/gxi-rs) project*
//!
//! This project covers all the widgets standardised in the
//! [GTK3 docs](https://developer.gnome.org/gtk3/stable/gtkobjects.html).

pub use containers::*;
pub use glib;
pub use gtk::prelude::*;
pub use run::*;
pub use tokio;
pub use widgets::*;

mod containers;
mod run;
mod widgets;
#[macro_use]
mod desktop_impl_macros;
