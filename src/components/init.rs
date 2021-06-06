use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::{*};

struct Init {
    child: Option<NodeType>,
    sibling: Option<NodeType>,
    parent: WeakNodeType,
}

impl Node for Init {
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

impl ComponentNode for Init {}

impl Init {
    fn new(parent: WeakNodeType, _constructor_params: ()) -> NodeType {
        NodeType::Component(Rc::new(RefCell::new(Box::new(Self {
            child: None,
            sibling: None,
            parent,
        }))))
    }
}
