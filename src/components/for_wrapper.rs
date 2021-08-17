use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::*;

pub struct ForWrapper<T>
where
    T: 'static,
{
    /// all children indexed with a key of type T
    pub children: HashMap<T, (StrongNodeType, bool)>,
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

    impl_node_trait_as_any!();
    impl_node_trait_as_node!();
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

impl<T> ForWrapper<T>
where
    T: std::hash::Hash + std::cmp::Eq + 'static,
{
    pub fn init_child(this: StrongNodeType, key: T) -> (StrongNodeType, bool) {
        let weak_this = Rc::downgrade(&this);

        let mut this_mut_borrow = this.as_ref().borrow_mut();
        let this_mut_borrow = this_mut_borrow
            .as_node_mut()
            .as_any_mut()
            .downcast_mut::<ForWrapper<T>>()
            .unwrap();

        let children = &mut this_mut_borrow.children;

        let is_new = children.contains_key(&key);

        let (node, existed) = children
            .entry(key)
            .or_insert_with(move || (Pure::new(weak_this), true));
        *existed = true;

        (node.clone(), is_new)
    }

    pub fn clear_unused(this: StrongNodeType) {
        let mut this_mut_borrow = this.as_ref().borrow_mut();
        let this_mut_borrow = this_mut_borrow
            .as_node_mut()
            .as_any_mut()
            .downcast_mut::<ForWrapper<T>>()
            .unwrap();
        this_mut_borrow.children.retain(|_k, v| {
            let existed = v.1.clone();
            v.1 = false;
            existed
        });
    }
}
