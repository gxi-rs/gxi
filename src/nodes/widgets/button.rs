use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::nodes::container::Container;
use crate::nodes::node::{Node, NodeTrait};
use crate::nodes::widget::Widget;

#[derive(Default)]
pub struct Button {
    pub child: Option<Arc<Mutex<Node>>>,
    pub sibling: Option<Arc<Mutex<Node>>>,
    pub parent: Option<Arc<Mutex<Node>>>,
}

impl NodeTrait for Button {
    impl_node_trait!();
}

impl Widget for Button {}
