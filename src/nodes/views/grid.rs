use std::sync::{Arc, Mutex};

use crate::nodes::container::Container;
use crate::nodes::node::{AsyncNode, Node, NodeTrait};

#[derive(Default)]
pub struct Grid {
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub parent: Option<AsyncNode>,
}

impl Container for Grid {}

impl NodeTrait for Grid {
    impl_node_trait!();
}
