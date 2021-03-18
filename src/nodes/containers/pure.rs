use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::{ContainerExt, Orientation};

use crate::nodes::node::{AsyncNode, Node};

pub struct Pure {
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub parent: AsyncNode,
}

impl Node for Pure {
    impl_node_trait!();
    impl_node_trait_init_sibling!();
}

impl Pure {
    pub fn new(parent: AsyncNode) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(Pure {
            child: None,
            sibling: None,
            parent,
        })))
    }
}
