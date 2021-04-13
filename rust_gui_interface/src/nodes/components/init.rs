use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::nodes::node::*;

pub struct Init {
    pub parent: WeakNodeRc<Self>,
    pub dirty: bool,
    pub self_substitute: Option<WeakNodeRc<Self>>,
    pub child: Option<NodeRc<Self>>,
    pub sibling: Option<NodeRc<Self>>,
}

impl Node for Init {
    type NativeWidget = ();
    type NativeWidgetContainer = ();
    impl_node_for_component!();

    fn new(parent: WeakNodeRc<Self>) -> NodeRc<Self> {
        let this: NodeRc<Self> = Rc::new(RefCell::new(Box::new(Self {
            parent,
            dirty: true,
            self_substitute: None,
            child: None,
            sibling: None,
        })));
        {
            let mut this_borrow = this.as_ref().borrow_mut();
            this_borrow.set_self_substitute(this.clone());
        }
        this
    }
}

impl Init {
    pub fn on_init<F: FnOnce() + 'static>(&self, f: F) {
        f();
    }
}

impl_drop_for_component!(Init);
