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

    fn init_child(&mut self, f: Box<dyn Fn() -> AsyncNode>) -> &mut AsyncNode {
        match self.child {
            None => {
                let child = self.child.get_or_insert_with(|| f());
                self.widget.add(child.clone().borrow_mut().get_widget());
                child
            }
            _ => self.child.as_mut().unwrap(),
        }
    }
    fn init_sibling(&mut self, f: Box<dyn Fn() -> AsyncNode>) -> &mut AsyncNode {
        match self.sibling {
            None => {
                let sibling = self.sibling.get_or_insert_with(|| f());
                self.widget.add(sibling.clone().borrow_mut().get_widget());
                sibling
            }
            _ => self.sibling.as_mut().unwrap(),
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
