

use crate::nodes::container::Container;
use crate::nodes::node::{AsyncNode, NodeTrait};
use std::any::Any;

pub struct Grid {
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub parent: AsyncNode,
    pub widget: gtk::Grid,
}

impl Container for Grid {
    fn get_widget(&self) -> &gtk::Container {
        self.widget.as_ref()
    }
}

impl Drop for Grid {
    fn drop(&mut self) {}
}

impl NodeTrait for Grid {
    impl_node_trait!();
}

impl Grid {
    pub fn new(parent: AsyncNode) -> Self {
        Grid {
            child: None,
            sibling: None,
            parent,
            widget: gtk::Grid::new(),
        }
    }
}
