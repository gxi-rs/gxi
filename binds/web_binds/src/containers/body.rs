use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use rust_gui_interface::{Node, NodeRc, WeakNodeRc};

use crate::*;

pub struct Body {
    pub parent: WeakNodeRc,
    pub dirty: bool,
    pub self_substitute: Option<WeakNodeRc>,
    pub child: Option<NodeRc>,
    pub widget: web_sys::HtmlElement,
}

impl Node for Body {
    impl_node_trait!();
    impl_node_trait_get_widget!();
    impl_node_trait_get_widget_as_container!();
    impl_node_trait_init_child!();
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

    fn render(state: NodeRc) {
        let mut state = state.as_ref().borrow_mut();
        let state = state.as_any_mut().downcast_mut::<Self>().unwrap();
        if state.dirty {
            //
        }
        state.mark_clean();
    }

    fn add(&mut self, child: NodeRc) {
        self.widget.append_child(&child.as_ref().borrow().get_widget()).unwrap();
        self.mark_dirty();
    }
}

//impl_drop_for_node!(Window);
impl Drop for Body {
    fn drop(&mut self) {
        // self.widget.parent_node().unwrap().remove_child(&self.widget);
    }
}
/*
let window: Window = web_sys::window().unwrap();
        let document: Document = window.document().unwrap();
        let body: HtmlElement = document.body().expect("document should have a body");
        let val:Element = document.create_element("p").unwrap();
        val.set_inner_html("<div>Hello</div>");
        body.append_child(&val).unwrap();
        */