use crate::{NodeType, WeakGxiNodeType, GxiNodeRc, Node};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

struct Init {
    child: Option<NodeType>,
    sibling: Option<NodeType>,
    parent: WeakGxiNodeType,
}

impl Node for Init {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn render(_this: GxiNodeRc) {}

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

impl Init {
    fn new(parent: WeakGxiNodeType, _constructor_params: ()) -> NodeType {
        NodeType::Component(Rc::new(RefCell::new(Box::new(Self {
            child: None,
            sibling: None,
            parent,
        }))))
    }
}
