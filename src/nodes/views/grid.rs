use std::cell::RefCell;
use std::rc::Rc;

use crate::nodes::container::Container;
use crate::nodes::node::{Node, NodeTrait};

#[derive(Default)]
pub struct Grid {
    pub child: Option<Rc<RefCell<Node>>>,
    pub sibling: Option<Rc<RefCell<Node>>>,
    pub parent: Option<Rc<RefCell<Node>>>,
}

impl Container for Grid {}

impl NodeTrait for Grid {
    impl_node_trait!();
}
