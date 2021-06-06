use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::*;

pub(crate) struct Foo {
    child: Option<NodeType>,
    sibling: Option<NodeType>,
    parent: WeakNodeType,
}

impl Node for Foo {
    fn new(parent: WeakNodeType) -> NodeType where Self: Sized {
        NodeType::Component(Rc::new(RefCell::new(Box::new(Self {
            child: None,
            sibling: None,
            parent,
        }))))
    }

    impl_node_trait_as_any!();
    impl_node_member_getters!();
}

impl ComponentNode for Foo {}

impl Drop for Foo {
    fn drop(&mut self) {}
}
