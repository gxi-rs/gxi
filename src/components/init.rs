use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::*;

struct Init {
    child: Option<GxiNodeType>,
    sibling: Option<GxiNodeType>,
    parent: WeakGxiNodeType,
}

impl GxiNode for Init {
    fn new(parent: WeakGxiNodeType) -> GxiNodeType {
        GxiNodeType::Component(Rc::new(RefCell::new(Box::new(Self {
            child: None,
            sibling: None,
            parent,
        }))))
    }

    impl_node_trait_as_any!();
    impl_node_getters!();
}

impl_container!(Init);
impl ComponentNode for Init {}

impl Init {
    fn new(parent: WeakGxiNodeType, _constructor_params: ()) -> GxiNodeType {
        GxiNodeType::Component(Rc::new(RefCell::new(Box::new(Self {
            child: None,
            sibling: None,
            parent,
        }))))
    }
}
