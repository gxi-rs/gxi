use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::{WidgetExt, WindowType};

use crate::*;
use gxi::*;

pub struct Window {
    parent: WeakGxiNodeType,
    child: Option<GxiNodeType>,
    sibling: Option<GxiNodeType>,
    widget: GtkContainer<gtk::Window>,
}

impl GxiNode for Window {
    fn new(parent: WeakGxiNodeType) -> GxiNodeType {
        GxiNodeType::Widget(Rc::new(RefCell::new(Box::new(Self {
            parent,
            child: None,
            sibling: None,
            widget: GtkContainer(gtk::Window::new(WindowType::Toplevel)),
        }))))
    }

    impl_node_trait_as_any!();
    impl_node_member_getters!();
}

impl_widget_node!(Window);

impl Window {
    pub fn on_destroy<F: Fn() + 'static>(&self, f: F) {
        self.widget.0.connect_destroy(move |_| f());
    }
}

impl_drop!(Window);
