use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::{WidgetExt, WindowType};

use crate::*;
use gxi::*;

pub struct Window {
    parent: WeakNodeType,
    child: Option<StrongNodeType>,
    sibling: Option<StrongNodeType>,
    widget: GtkContainer<gtk::Window>,
    self_substitute: Option<WeakNodeType>,
}

impl Node for Window {
    fn new(parent: WeakNodeType) -> StrongNodeType {
        Rc::new(RefCell::new(GxiNodeType::Container(Box::new(Self {
            parent,
            child: None,
            sibling: None,
            widget: GtkContainer({
                let window = gtk::Window::new(WindowType::Toplevel);
                window.show_all();
                window
            }),
            self_substitute: None
        }))))
    }

    impl_node_trait_as_any!();
    impl_node_trait_as_node!();
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
