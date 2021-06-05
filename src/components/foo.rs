use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::interface::{NodeType, WeakGxiNodeType, GxiNodeRc, Node};

pub(crate) struct Foo {
    child: Option<NodeType>,
    sibling: Option<NodeType>,
    parent: WeakGxiNodeType,
}

impl Node for Foo {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn render(_this: GxiNodeRc) {
        todo!()
    }

    fn get_child(&self) -> &Option<NodeType> {
        &self.child
    }

    fn get_child_mut(&mut self) -> &mut Option<NodeType> {
        &mut self.child
    }

    fn get_parent(&self) -> WeakGxiNodeType {
        self.parent.clone()
    }
}

impl Foo {
    pub(crate) fn new(parent: WeakGxiNodeType, _construct_values: ()) -> NodeType {
        NodeType::Component(Rc::new(RefCell::new(Box::new(Self {
            child: None,
            sibling: None,
            parent,
        }))))
    }
}

impl Drop for Foo {
    fn drop(&mut self) {}
}
