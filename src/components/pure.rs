use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::*;

struct Pure {
    /// index of current if block where 0 is default i.e no if block has been executed yet
    pub pure_index: u32,
    // others
    child: Option<GxiNodeType>,
    sibling: Option<GxiNodeType>,
    parent: WeakGxiNodeType,
    self_substitute: Option<WeakGxiNodeType>,
}

impl GxiNode for Pure {
    fn new(parent: WeakGxiNodeType) -> GxiNodeType {
        GxiNodeType::Component(Rc::new(RefCell::new(Box::new(Self {
            pure_index: 0,
            child: None,
            sibling: None,
            parent,
            self_substitute: None
        }))))
    }

    impl_node_trait_as_any!();
    impl_node_getters!();
}

impl_container!(Pure);
impl_component_node!(Pure);

impl Pure {
    fn new(parent: WeakGxiNodeType, _constructor_params: ()) -> GxiNodeType {
        GxiNodeType::Component(Rc::new(RefCell::new(Box::new(Self {
            pure_index: 0,
            child: None,
            sibling: None,
            parent,
            self_substitute: None
        }))))
    }
}
