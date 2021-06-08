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
    self_substitute: Option<WeakGxiNodeType>,
}

impl Node for Window {
    fn new(parent: WeakGxiNodeType) -> GxiNodeType {
        GxiNodeType::Container(Rc::new(RefCell::new(Box::new(Self {
            parent,
            child: None,
            sibling: None,
            widget: GtkContainer(gtk::Window::new(WindowType::Toplevel)),
            self_substitute: None
        }))))
    }

    impl_node_trait_as_any!();
    impl_node_getters!();
}

impl ContainerNode for Window {}
impl_component_node!(Window);
impl_container!(Window);
impl_widget_node!(Window);

impl Window {
    pub fn on_destroy<F: Fn() + 'static>(&self, f: F) {
        self.widget.0.connect_destroy(move |_| f());
    }
}

impl_drop!(Window);
