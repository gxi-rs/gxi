use std::any::Any;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

use gtk::ContainerExt;

use crate::nodes::node::{AsyncNode, Node};
use crate::nodes::widget::Widget;

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
        Arc::new(Mutex::new(Box::new(Button {
            widget: gtk::Button::new(),
            child: None,
            sibling: None,
            parent,
        })))
    }
}
