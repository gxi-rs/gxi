use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::nodes::node::{AsyncNode, Node};

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
        Rc::new(RefCell::new(Box::new(Grid {
            child: None,
            sibling: None,
            parent,
            widget: gtk::Grid::new(),
        })))
    }
}
