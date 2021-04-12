use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::{prelude::*, WindowType};

use crate::nodes::node::{Node, NodeRc, NodeType};
use crate::{*};

pub struct Window {
    pub parent: WeakNodeRc,
    pub dirty: bool,
    pub self_substitute: Option<WeakNodeRc>,
    pub child: Option<NodeRc>,
    pub widget: gtk::Window,
}

impl Node for Window {
    impl_node_trait!();
    impl_node_trait_get_widget!();
    impl_node_trait_get_widget_as_container!();
    impl_node_trait_init_child!();
    impl_node_trait_add!();
    impl_node_trait_substitute!();

    fn new(parent: WeakNodeRc) -> NodeRc {
        let this: NodeRc = Rc::new(RefCell::new(Box::new(Self {
            parent,
            dirty: true,
            self_substitute: None,
            child: None,
            widget: gtk::Window::new(WindowType::Toplevel),
        })));
        {
            let mut this_borrow = this.as_ref().borrow_mut();
            this_borrow.set_self_substitute(this.clone());
        }
        this
    }

    fn init_sibling(&mut self, _f: Box<dyn FnOnce() -> NodeRc>) -> (NodeRc, bool) {
        panic!("Window can't have a sibling node");
    }
}

impl_drop_for_node!(Window);
