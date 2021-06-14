use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::*;

pub struct Pure {
    /// index of current if block where 0 is default i.e no if block has been executed yet
    pub pure_index: u32,
    // pub , accessed by get_pure_remove_block
    pub child: Option<StrongNodeType>,
    sibling: Option<StrongNodeType>,
    parent: WeakNodeType,
    self_substitute: Option<WeakNodeType>,
}

impl Node for Pure {
    fn new(parent: WeakNodeType) -> StrongNodeType {
        let this = Rc::new(RefCell::new(GxiNodeType::Component(Box::new(Self {
            pure_index: 0,
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

impl_container!(Pure);
impl_component_node!(Pure);
