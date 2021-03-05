use std::cell::RefCell;
use std::rc::Rc;

use crate::nodes::container::Container;
use crate::nodes::node::{Node, NodeTrait};
use crate::nodes::widget::Widget;

pub struct Button {
    pub parent: Rc<RefCell<Box<dyn Container>>>,
}

impl NodeTrait for Button {
    fn get_child(&self) -> &Option<Node> {
        unimplemented!()
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

impl Widget for Button {}
