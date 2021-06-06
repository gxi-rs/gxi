use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::*;

struct Init {
    child: Option<GxiNodeType>,
    sibling: Option<GxiNodeType>,
    parent: WeakGxiNodeType,
    self_substitute: Option<WeakGxiNodeType>,
}

impl GxiNode for Init {
    fn new(parent: WeakGxiNodeType) -> GxiNodeType {
        GxiNodeType::Component(Rc::new(RefCell::new(Box::new(Self {
            child: None,
            sibling: None,
            parent,
            self_substitute: None,
        }))))
    }

    impl_node_trait_as_any!();
    impl_node_getters!();
}

impl_container!(Init);
impl_component_node!(Init);

impl Init {
    fn new(parent: WeakGxiNodeType, _constructor_params: ()) -> GxiNodeType {
        GxiNodeType::Component(Rc::new(RefCell::new(Box::new(Self {
            self_substitute: None,
            child: None,
            sibling: None,
            parent,
        }))))
    }
}
