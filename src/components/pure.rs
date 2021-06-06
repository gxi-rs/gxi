use crate::*;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

struct Pure {
    /// index of current if block where 0 is default i.e no if block has been executed yet
    pub pure_index: u32,
    // others
    child: Option<NodeType>,
    sibling: Option<NodeType>,
    parent: WeakNodeType,
}

impl Node for Pure {
    fn new(parent: WeakNodeType) -> NodeType where Self: Sized {
        NodeType::Component(Rc::new(RefCell::new(Box::new(Self {
            pure_index: 0,
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

impl ComponentNode for Pure {}

impl Pure {
    fn new(parent: WeakNodeType, _constructor_params: ()) -> NodeType {
        NodeType::Component(Rc::new(RefCell::new(Box::new(Self {
            pure_index: 0,
            child: None,
            sibling: None,
            parent,
        }))))
    }
}
