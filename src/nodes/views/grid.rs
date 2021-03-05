use crate::nodes::container::Container;
use crate::nodes::node::{Node, NodeTrait};

#[derive(Default)]
pub struct Grid {
    pub child: Option<Node>,
}

impl Container for Grid {}

impl NodeTrait for Grid {
    fn get_child(&self) -> &Option<Node> {
        &self.child
    }

    fn get_sibling(&self) -> &Option<Node> {
        unimplemented!()
    }

    fn get_child_mut(&mut self) -> &mut Option<Node> {
        unimplemented!()
    }

    fn get_sibling_mut(&mut self) -> &mut Option<Node> {
        unimplemented!()
    }
}
