use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::{ContainerExt, WindowType};

use crate::nodes::node::{AsyncNode, Node};

pub struct Window {
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub widget: gtk::Window,
}

impl Node for Window {
    impl_node_trait!();
    impl_node_trait_init_child!();
    impl_node_trait_get_widget!();
}

impl Window {
    pub fn new(window_type: WindowType) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(Window {
            child: None,
            sibling: None,
            widget: gtk::Window::new(window_type),
        })))
    }
}
