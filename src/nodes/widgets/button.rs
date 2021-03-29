use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;

use crate::nodes::node::{AsyncNode, Node, NodeType};

pub struct Button {
    pub widget: gtk::Button,
    pub sibling: Option<AsyncNode>,
}

impl Node for Button {
    impl_node_trait!();
    impl_node_trait_init_sibling!();
    impl_node_trait_get_widget!();
    impl_node_trait_get_sibling!();

    fn init_child(&mut self, _f: Box<dyn FnOnce() -> AsyncNode>, parent: AsyncNode) -> (AsyncNode, bool) {
        panic!("Attempt to a add node into Button. Button can't have a child.");
    }

    fn new(parent_widget: Option<gtk::Container>) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(Button {
            widget: gtk::Button::new(),
            sibling: None,
        })))
    }
}

impl_drop_for_node!(Button);