use gtk::{WindowType};

use crate::nodes::node::{AsyncNode, Node};
use std::any::Any;

pub struct Window {
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub widget: gtk::Window,
}

impl Node for Window {
    impl_node_trait!();
}

impl Window {
    pub fn new(window_type: WindowType) -> Self {
        Window {
            child: None,
            sibling: None,
            widget: gtk::Window::new(window_type),
        }
    }
}
