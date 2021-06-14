use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::*;

pub struct Root {
    child: Option<StrongNodeType>,
}

impl Node for Root {
    fn new(_parent: WeakNodeType) -> StrongNodeType {
        Self::new_root()
    }

    impl_node_trait_as_any!();
    impl_node_trait_as_node!();

    fn get_parent(&self) -> &WeakNodeType {
        panic!("can't get parent of tree root. You have likely forgotten to add a widget container in your tree. Eg. Window, Body")
    }

    fn get_sibling(&self) -> &Option<StrongNodeType> { unreachable!() }

    fn get_sibling_mut(&mut self) -> &mut Option<StrongNodeType> { unreachable!() }
    
}

impl_container!(Root);

impl ComponentNode for Root {
    fn get_self_substitute(&self) -> &Option<WeakNodeType> { unreachable!() }

    fn get_self_substitute_mut(&mut self) -> &mut Option<WeakNodeType> { unreachable!() }
}

impl Root {
    pub fn new_root() -> StrongNodeType {
        Rc::new(RefCell::new(GxiNodeType::Component(Box::new(Self {
            child: None
        }))))
    }
}

impl Drop for Root {
    fn drop(&mut self) {}
}
