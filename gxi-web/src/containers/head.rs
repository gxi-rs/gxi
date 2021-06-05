use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::*;

pub struct Head {
    pub parent: WeakNodeRc,
    pub self_substitute: Option<WeakNodeRc>,
    pub child: Option<NodeRc>,
    pub sibling: Option<NodeRc>,
    pub widget: web_sys::HtmlHeadElement,
}

impl Node for Head {
    impl_node_for_widget_component!();

    fn new(parent: WeakNodeRc) -> NodeRc {
        let this: NodeRc = Rc::new(RefCell::new(Box::new(Self {
            parent,
            self_substitute: None,
            child: None,
            sibling: None,
            widget: {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                document.head().unwrap()
            },
        })));
        {
            let mut this_borrow = this.as_ref().borrow_mut();
            this_borrow.set_self_substitute(this.clone());
        }
        this
    }

    impl_add_for_web_node!();
}

impl GlobalAttributes for Head {}

impl Drop for Head {
    fn drop(&mut self) {
        //need not drop head tag
    }
}
