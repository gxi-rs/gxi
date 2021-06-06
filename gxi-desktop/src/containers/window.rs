use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::{WidgetExt, WindowType};

use gxi::*;

use crate::*;

pub struct Window {
    pub parent: WeakNodeType,
    pub child: Option<NodeType>,
    pub sibling: Option<NodeType>,
    pub widget: GtkElement<gtk::Window>,
}

impl Node for Window {
    fn new(parent: WeakNodeType) -> NodeType {
        NodeType::Widget(Rc::new(RefCell::new(Box::new(Self {
            parent,
            child: None,
            sibling: None,
            widget: GtkElement(gtk::Window::new(WindowType::Toplevel)),
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
