use std::sync::{Arc, Mutex};

use crate::nodes::container::Container;
use crate::nodes::node::{Node, NodeTrait};

#[derive(Default)]
pub struct Grid {
    pub child: Option<Arc<Mutex<Node>>>,
    pub sibling: Option<Arc<Mutex<Node>>>,
    pub parent: Option<Arc<Mutex<Node>>>,
}

impl Container for Grid {}

impl NodeTrait for Grid {
    impl_node_trait!();
}
