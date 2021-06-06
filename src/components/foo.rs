use crate::*;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

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

    fn get_child(&self) -> &Option<NodeType> {
        &self.child
    }

    fn get_child_mut(&mut self) -> &mut Option<NodeType> {
        &mut self.child
    }

    fn get_parent(&self) -> WeakNodeType {
        self.parent.clone()
    }
}

impl ComponentNode for Foo {}

impl Drop for Foo {
    fn drop(&mut self) {}
}
