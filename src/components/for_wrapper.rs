use crate::*;

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct ForWrapper<T>
where
    T: 'static,
{
    /// all children indexed with a key of type T
    pub children: HashMap<T, StrongNodeType>,
    sibling: Option<StrongNodeType>,
    parent: WeakNodeType,
}

impl<T> Node for ForWrapper<T>
where
    T: 'static,
{
    fn new(parent: WeakNodeType) -> StrongNodeType {
        Rc::new(RefCell::new(GxiNodeType::Component(Box::new(Self {
            children: HashMap::new(),
            sibling: None,
            parent,
        }))))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn as_node(&self) -> &dyn Node {
        self
    }
    fn as_node_mut(&mut self) -> &mut dyn Node {
        self
    }
    impl_node_getters!();
}

//TODO: add types for macros

const SUBST_ERR: &str =
    "can't get self_substitute of ForWrapper. Note: ForWrapper is meant for internal use only.";
impl<T> ComponentNode for ForWrapper<T> {
    fn get_self_substitute(&self) -> &Option<WeakNodeType> {
        panic!("{}", SUBST_ERR)
    }

    fn get_self_substitute_mut(&mut self) -> &mut Option<WeakNodeType> {
        panic!("{}", SUBST_ERR)
    }
}

const CHILD_ERR: &str =
    "ForWrapper has no direct child. Note: ForWrapper is meant for internal use only.";

impl<T> Container for ForWrapper<T> {
    fn get_child(&self) -> &Option<StrongNodeType> {
        panic!("{}", CHILD_ERR)
    }

    fn get_child_mut(&mut self) -> &mut Option<StrongNodeType> {
        panic!("{}", CHILD_ERR)
    }

    fn as_container(&self) -> &dyn Container {
        self
    }

    fn as_container_mut(&mut self) -> &mut dyn Container {
        self
    }
}

impl<T> ForWrapper<T> {
    pub fn init_child(&self, this: StrongNodeType, key: T) -> (StrongNodeType, bool) {
        unimplemented!();
    }
}
