use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub use containers::*;
pub use run::*;
pub use glib;
pub use gtk::prelude::*;
use rust_gui_interface::*;
pub use widgets::*;

mod containers;
mod widgets;
mod run;

pub type DesktopNode = Box<dyn Node<NativeWidgetContainer=gtk::Container, NativeWidget=gtk::Widget>>;
pub type DesktopNodeRc = Rc<RefCell<DesktopNode>>;
pub type DesktopWeakNodeRc = Weak<RefCell<DesktopNode>>;
