use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::nodes::node::*;

pub struct Init {
    pub parent: WeakNodeRc,
    pub self_substitute: Option<WeakNodeRc>,
    pub child: Option<NodeRc>,
    pub sibling: Option<NodeRc>,
}

impl Node for Init {
    impl_node_for_component!();

    fn new(parent: WeakNodeRc) -> NodeRc {
        let this: NodeRc = Rc::new(RefCell::new(Box::new(Self {
            parent,
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
