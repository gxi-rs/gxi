use std::any::Any;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};

use crate::nodes::node::{AsyncNode, Node};
use std::rc::Rc;

pub struct Grid {
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub parent: AsyncNode,
    pub widget: gtk::Grid,
}

impl Node for Grid {
    impl_node_trait!();
}

impl Grid {
    pub fn new(parent: AsyncNode) -> AsyncNode {
        Box::new(Grid {
            child: None,
            sibling: None,
            parent,
            widget: gtk::Grid::new(),
        })
    }
}
