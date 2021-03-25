use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::{prelude::*, WindowType};

use crate::nodes::node::{AsyncNode, Node};

pub struct Window {
    pub child: Option<AsyncNode>,
    pub widget: gtk::Window,
    pub parent: AsyncNode,
}

impl Node for Window {
    impl_node_trait!();
    impl_node_trait_get_widget!();
    impl_node_trait_init_child!();

    fn init_sibling(
        &mut self, _f: Box<dyn FnOnce() -> AsyncNode>, _add_widget: bool,
    ) -> (AsyncNode, bool) {
        panic!("Window can't have a sibling node");
    }

    fn new(parent: AsyncNode, _widget: Option<gtk::Container>) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(Window {
            child: None,
            widget: gtk::Window::new(WindowType::Toplevel),
            parent,
        })))
    }
}

