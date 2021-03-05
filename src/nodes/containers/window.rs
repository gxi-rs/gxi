use gtk::{ContainerExt, WindowType};

use crate::nodes::container::Container;
use crate::nodes::node::{AsyncNode, NodeTrait};
use std::any::Any;

pub struct Window {
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub widget: gtk::Window,
}

impl Container for Window {
    fn get_widget(&self) -> &gtk::Container {
        self.widget.as_ref()
    }
}

impl Drop for Window {
    fn drop(&mut self) {}
}

impl NodeTrait for Window {
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
