use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::{ContainerExt, Orientation};

use crate::nodes::node::{AsyncNode, Node};

pub struct View {
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub parent: AsyncNode,
    pub widget: gtk::Box,
}

impl Node for View {
    impl_node_trait!();
    impl_node_trait_init_sibling!();
    init_node_trait_child!();
}

impl View {
    pub fn new(parent: AsyncNode) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(View {
            child: None,
            sibling: None,
            parent,
            widget: gtk::Box::new(Orientation::Horizontal, 1),
        })))
    }
}