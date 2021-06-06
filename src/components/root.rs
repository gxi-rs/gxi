use crate::*;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Root {
    child: Option<NodeType>,
}

impl Node for Root {
    fn new(_parent: WeakNodeType) -> NodeType where Self: Sized {
        NodeType::Component(Rc::new(RefCell::new(Box::new(Self { child: None }))))
    }

    impl_node_trait_as_any!();

    fn get_child(&self) -> &Option<NodeType> {
        &self.child
    }

    fn get_child_mut(&mut self) -> &mut Option<NodeType> {
        &mut self.child
    }

    fn get_parent(&self) -> WeakNodeType { unreachable!() }
}

impl ComponentNode for Root {}

impl Root {
    pub fn new_root() -> NodeType {
        NodeType::Component(Rc::new(RefCell::new(Box::new(Self { child: None }))))
    }
}

impl Drop for Root {
    fn drop(&mut self) {}
}
