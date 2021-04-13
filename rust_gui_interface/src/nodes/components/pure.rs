use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::nodes::node::*;

pub struct Pure {
    pub parent: WeakNodeRc<Self>,
    pub dirty: bool,
    pub self_substitute: Option<WeakNodeRc<Self>>,
    pub child: Option<NodeRc<Self>>,
    pub sibling: Option<NodeRc<Self>>,
    pub pure_index: u32, //Index of current if block where 0 is default i.e when no if block was rendered before
}

impl Node for Pure {
    type NativeWidget = ();
    type NativeWidgetContainer = ();
    impl_node_for_component!();

    fn new(parent: WeakNodeRc<Self>) -> NodeRc<Self> {
        let this: NodeRc<Self> = Rc::new(RefCell::new(Box::new(Self {
            parent,
            dirty: true,
            pure_index: 0,
            child: None,
            sibling: None,
            self_substitute: None,
        })));
        {
            let mut this_borrow = this.as_ref().borrow_mut();
            this_borrow.set_self_substitute(this.clone());
        }
        this
    }
}

impl_drop_for_component!(Pure);
