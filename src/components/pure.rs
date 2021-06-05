use crate::{NodeType, WeakGxiNodeType, GxiNodeRc, Node};
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

struct Pure {
    /// index of current if block where 0 is default i.e no if block has been executed yet
    pub pure_index: u32,
    // others
    child: Option<NodeType>,
    sibling: Option<NodeType>,
    parent: WeakGxiNodeType,
}

impl Node for Pure {
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
        self.parent.clone()
    }
}

impl Pure {
    fn new(parent: WeakGxiNodeType, _constructor_params: ()) -> NodeType {
        NodeType::Component(Rc::new(RefCell::new(Box::new(Self {
            pure_index: 0,
            child: None,
            sibling: None,
            parent,
        }))))
    }
}
