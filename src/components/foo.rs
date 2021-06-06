use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::*;

pub(crate) struct Foo {
    child: Option<GxiNodeType>,
    sibling: Option<GxiNodeType>,
    parent: WeakGxiNodeType,
}

impl GxiNode for Foo {
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

impl_container!(Foo);
impl ComponentNode for Foo {}

impl Drop for Foo {
    fn drop(&mut self) {}
}
