use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::*;

pub struct Body {
    pub parent: WeakNodeRc,
    pub self_substitute: Option<WeakNodeRc>,
    pub child: Option<NodeRc>,
    pub sibling: Option<NodeRc>,
    pub widget: web_sys::HtmlElement,
}

impl Node for Body {
    impl_node_as_any!();
    impl_node_trait_get_child!();
    impl_node_trait_get_sibling!();
    impl_node_trait_init_sibling!();
    impl_node_trait_substitute!();
    impl_node_trait_get_widget!();
    impl_node_trait_init_child!();

    fn get_type(&self) -> NodeType {
        NodeType::Component
    }

    fn new(parent: WeakNodeRc) -> NodeRc {
        let this: NodeRc = Rc::new(RefCell::new(Box::new(Self {
            parent,
            self_substitute: None,
            child: None,
            sibling: None,
            widget: {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                document.body().unwrap()
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

impl GlobalAttributes for Body {}

impl Drop for Body {
    fn drop(&mut self) {
        //need not drop body tag
    }
}
