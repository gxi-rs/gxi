use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::*;

pub(crate) struct Foo {
    child: Option<StrongNodeType>,
    sibling: Option<StrongNodeType>,
    parent: WeakNodeType,
    self_substitute: Option<WeakNodeType>,
}

impl Node for Foo {
    fn new(parent: WeakNodeType) -> StrongNodeType {
        Rc::new(RefCell::new(GxiNodeType::Component(Box::new(Self {
            child: None,
            sibling: None,
            parent,
            self_substitute: None,
        }))))
    }

    impl_node_trait_as_any!();
    impl_node_trait_as_node!();
    impl_node_getters!();
}

impl_container!(Foo);
impl_component_node!(Foo);
