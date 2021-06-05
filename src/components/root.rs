use crate::interface::{Node, GxiNodeRc, NodeType, WeakGxiNodeType};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Root {
    child: Option<NodeType>,
}

impl Node for Root {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_child(&self) -> &Option<NodeType> {
        &self.child
    }

    fn get_child_mut(&mut self) -> &mut Option<NodeType> {
        &mut self.child
    }

    fn get_parent(&self) -> WeakGxiNodeType {
        unreachable!()
    }
}

impl Root {
    pub fn new_root() -> NodeType {
        NodeType::Component(Rc::new(RefCell::new(Box::new(Self { child: None }))))
    }
}

impl Drop for Root {
    fn drop(&mut self) {}
}
