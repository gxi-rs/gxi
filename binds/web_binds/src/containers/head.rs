use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use rust_gui_interface::{Node, NodeRc, WeakNodeRc};

use crate::*;

pub struct Head {
    pub parent: WeakNodeRc,
    pub dirty: bool,
    pub self_substitute: Option<WeakNodeRc>,
    pub child: Option<NodeRc>,
    pub widget: web_sys::HtmlElement,
}

impl Node for Head {
    impl_node_as_any!();
    impl_node_trait_get_widget!();
    impl_node_trait_get_widget_as_container!();
    impl_node_trait_init_child!();
    impl_node_trait_substitute!();

    fn init_sibling(&mut self, _f: Box<dyn FnOnce() -> NodeRc>) -> (NodeRc, bool) {
        panic!("Window can't have a.rs sibling node");
    }

    fn new(parent: WeakNodeRc) -> NodeRc {
        let this: NodeRc = Rc::new(RefCell::new(Box::new(Self {
            parent,
            dirty: false,
            self_substitute: None,
            child: None,
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

    fn add(&mut self, child: NodeRc) {
        self.widget
            .append_child(&child.as_ref().borrow().get_widget())
            .unwrap();
    }
}

impl Drop for Head {
    fn drop(&mut self) {
        self.widget
            .parent_node()
            .unwrap()
            .remove_child(&self.widget)
            .unwrap();
    }
}
