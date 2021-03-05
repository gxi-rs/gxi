use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::nodes::container::Container;
use crate::nodes::node::{AsyncNode, Node, NodeTrait};
use crate::nodes::widget::Widget;

#[derive(Default)]
pub struct Button {
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub parent: Option<AsyncNode>,
}

impl NodeTrait for Button {
    impl_node_trait!();
}

impl Widget for Button {}
