use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::{prelude::*, WindowType};

use crate::*;
use rust_gui_interface::{Node, NodeRc, WeakNodeRc};

pub struct Window {
    pub parent: WeakNodeRc<Self>,
    pub dirty: bool,
    pub self_substitute: Option<WeakNodeRc<Self>>,
    pub child: Option<NodeRc<Self>>,
    pub widget: gtk::Window,
}

impl Node for Window {
    type NativeWidget = gtk::Widget;
    type NativeWidgetContainer = gtk::Container;
    impl_node_trait!();
    impl_node_trait_get_widget!();
    impl_node_trait_get_widget_as_container!();
    impl_node_trait_init_child!();
    impl_node_trait_add!();
    impl_node_trait_substitute!();

    fn init_sibling(&mut self, _f: Box<dyn FnOnce() -> NodeRc<Self>>) -> (NodeRc<Self>, bool) {
        panic!("Window can't have a sibling node");
    }

    fn new(parent: WeakNodeRc<Self>) -> NodeRc<Self> {
        let this: NodeRc<Self> = Rc::new(RefCell::new(Box::new(Self {
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
}

impl_drop_for_node!(Window);
