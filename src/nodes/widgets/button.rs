use std::any::Any;
use std::cell::RefCell;

use std::rc::Rc;

use crate::nodes::node::{AsyncNode, Node};

pub struct Button {
    pub widget: gtk::Button,
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub parent: AsyncNode,
}

impl Node for Button {
    impl_node_trait!();
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
