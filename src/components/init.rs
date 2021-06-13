use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::*;

pub struct Init {
    child: Option<StrongNodeType>,
    sibling: Option<StrongNodeType>,
    parent: WeakNodeType,
    self_substitute: Option<WeakNodeType>,
}

impl Node for Init {
    fn new(parent: WeakNodeType) -> StrongNodeType {
        let this = Rc::new(RefCell::new(GxiNodeType::Component(Box::new(Self {
            child: None,
            sibling: None,
            parent,
            self_substitute: None,
        }))));
        {
            let mut this_borrow = this.as_ref().borrow_mut();
            *this_borrow
                .as_component_node_mut()
                .unwrap()
                .get_self_substitute_mut() = Some(Rc::downgrade(&this));
        }
        this
    }

    impl_node_trait_as_any!();
    impl_node_trait_as_node!();
    impl_node_getters!();
}

impl Init {
    pub fn on_init<F: FnOnce() + 'static>(&self, f: F) {
        f();
    }
}

impl_container!(Init);
impl_component_node!(Init);
