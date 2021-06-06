use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::*;

pub struct Root {
    child: Option<GxiNodeType>,
}

impl GxiNode for Root {
    fn new(_parent: WeakGxiNodeType) -> GxiNodeType
        where
            Self: Sized,
    {
        GxiNodeType::Component(Rc::new(RefCell::new(Box::new(Self { child: None }))))
    }

    impl_node_trait_as_any!();

    fn get_parent(&self) -> &WeakGxiNodeType {
        unreachable!()
    }

    fn get_sibling(&self) -> &Option<GxiNodeType> { unreachable!() }

    fn get_sibling_mut(&mut self) -> &mut Option<GxiNodeType> { unreachable!() }
}

impl_container!(Root);

impl ComponentNode for Root {}

impl Root {
    pub fn new_root() -> GxiNodeType {
        GxiNodeType::Component(Rc::new(RefCell::new(Box::new(Self { child: None }))))
    }
}

impl Drop for Root {
    fn drop(&mut self) {}
}
