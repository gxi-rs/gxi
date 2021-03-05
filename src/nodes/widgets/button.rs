use std::cell::RefCell;
use std::rc::Rc;

use crate::nodes::container::Container;
use crate::nodes::node::{Node, NodeTrait};
use crate::nodes::widget::Widget;

#[derive(Default)]
pub struct Button {
    pub child: Option<Rc<RefCell<Node>>>,
    pub sibling: Option<Rc<RefCell<Node>>>,
    pub parent: Option<Rc<RefCell<Node>>>,
}

impl NodeTrait for Button {
    impl_node_trait!();
}

impl Widget for Button {}
