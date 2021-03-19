use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::{ContainerExt, WindowType};

use crate::nodes::node::{AsyncNode, Node};

pub struct Window {
    pub child: Option<AsyncNode>,
    pub widget: gtk::Window,
}

impl Node for Window {
    impl_node_trait!();
    impl_node_trait_get_widget!();
    //impl_node_trait_init_child!();
    fn init_child(&mut self, f: Box<dyn Fn() -> AsyncNode>) -> (AsyncNode, bool) {
        match self.child {
            None => {
                let child = self.child.get_or_insert_with(|| f());
                {
                    let child_borrow = child.as_ref().borrow();
                    self.widget.add(&child_borrow.get_widget());
                }
                (child.clone(), true)
            }
            _ => (self.child.as_ref().unwrap().clone(), false),
        }
    }
}

impl Window {
    pub fn new(window_type: WindowType) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(Window {
            child: None,
            widget: gtk::Window::new(window_type),
        })))
    }
}
