use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::{WindowType};

use crate::*;
use gxi::*;

pub struct Window {
    parent: WeakNodeType,
    child: Option<StrongNodeType>,
    sibling: Option<StrongNodeType>,
    widget: gtk::Window,
    self_substitute: Option<WeakNodeType>,
}

impl Node for Window {
    fn new(parent: WeakNodeType) -> StrongNodeType {
        Rc::new(RefCell::new(GxiNodeType::Container(Box::new(Self {
            parent,
            child: None,
            sibling: None,
            widget: gtk::Window::new(WindowType::Toplevel),
            self_substitute: None,
        }))))
    }

    impl_node_trait_as_any!();
    impl_node_trait_as_node!();
    impl_node_getters!();
}

impl ContainerNode for Window {
    fn get_native_container(&self) -> Box<dyn NativeContainer> {
        &self.widget
    }

}
impl_component_node!(Window);
impl_container!(Window);
impl_widget_node!(Window);

impl Window {
    pub fn on_destroy<F: Fn() + 'static>(&self, f: F) {
        self.widget.0.as_ref().connect_destroy(f);
    }
}

impl_drop!(Window);
