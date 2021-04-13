use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use rust_gui_interface::{NodeRc, WeakNodeRc, Node};

use crate::*;

pub struct Window {
    pub parent: WeakNodeRc,
    pub dirty: bool,
    pub self_substitute: Option<WeakNodeRc>,
    pub child: Option<NodeRc>,
    pub widget: web_sys::Window,
}

impl Node for Window {
    impl_node_trait!();
    impl_node_trait_get_widget!();
    impl_node_trait_get_widget_as_container!();
    impl_node_trait_init_child!();
    impl_node_trait_add!();
    impl_node_trait_substitute!();

    fn init_sibling(&mut self, _f: Box<dyn FnOnce() -> NodeRc>) -> (NodeRc, bool) {
        panic!("Window can't have a sibling node");
    }

    fn new(parent: WeakNodeRc) -> NodeRc {
        let this: NodeRc = Rc::new(RefCell::new(Box::new(Self {
            parent,
            dirty: true,
            self_substitute: None,
            child: None,
            widget: web_sys::window().unwrap(),
        })));
        {
            let mut this_borrow = this.as_ref().borrow_mut();
            this_borrow.set_self_substitute(this.clone());
        }
        this
    }
}

impl_drop_for_node!(Window);
/*
let window: Window = web_sys::window().unwrap();
        let document: Document = window.document().unwrap();
        let body: HtmlElement = document.body().expect("document should have a body");
        let val:Element = document.create_element("p").unwrap();
        val.set_inner_html("<div>Hello</div>");
        body.append_child(&val).unwrap();
        */