use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::ContainerExt;

use crate::nodes::node::{AsyncNode, Node};

pub struct Button {
    pub widget: gtk::Button,
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub parent: AsyncNode,
}

impl Node for Button {
    impl_node_trait!();
    fn init_sibling(&mut self, f: Box<dyn Fn() -> AsyncNode>) -> AsyncNode {
        match self.sibling {
            None => {
                let sibling = self.sibling.get_or_insert_with(|| f());
                self.parent
                    .borrow_mut()
                    .get_widget_as_container()
                    .add(sibling.clone().borrow_mut().get_widget());
                sibling.clone()
            }
            _ => self.sibling.as_ref().unwrap().clone(),
        }
    }
}

impl Button {
    pub fn new(parent: AsyncNode) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(Button {
            widget: gtk::Button::new(),
            child: None,
            sibling: None,
            parent,
        })))
    }
}
