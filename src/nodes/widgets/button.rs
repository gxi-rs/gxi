use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::ContainerExt;

use crate::nodes::node::{AsyncNode, Node};

pub struct Button {
    pub widget: gtk::Button,
    pub sibling: Option<AsyncNode>,
    pub parent: AsyncNode,
}

impl Node for Button {
    impl_node_trait!();
    impl_node_trait_init_sibling!();
    impl_node_trait_get_widget!();
}

impl Button {
    pub fn new(parent: AsyncNode) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(Button {
            widget: gtk::Button::new(),
            sibling: None,
            parent,
        })))
    }
}

impl Drop for Button {
    fn drop(&mut self) {
        println!("Dropping")
    }
}
