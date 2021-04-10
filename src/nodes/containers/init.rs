use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::nodes::node::*;

pub struct Init {
    pub parent: WeakNodeRc,
    pub dirty: bool,
    pub child: Option<NodeRc>,
    pub sibling: Option<NodeRc>,
}

impl Node for Init {
    impl_node_for_component!();
    fn new(parent: WeakNodeRc) -> NodeRc {
        Rc::new(RefCell::new(Box::new(Init {
            parent,
            dirty: true,
            child: None,
            sibling: None,
        })))
    }
}

impl Init {
    pub fn on_init<F: FnOnce() + 'static>(&self, f: F) {
        f();
    }
}

impl_drop_for_component!(Init);
