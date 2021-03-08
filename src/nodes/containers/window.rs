use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::{WindowType, ContainerExt};

use crate::nodes::node::{AsyncNode, Node};

pub struct Window {
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub widget: gtk::Window,
}

impl Node for Window {
    impl_node_trait!();

    fn init_child(&mut self, f: Box<dyn Fn() -> AsyncNode>) {
        match self.child {
            None => {
                self.child.replace(f());
                self.widget.add(self.child.unwrap().get_mut().get_widget());
            }
            _ => {}
        }
    }
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
