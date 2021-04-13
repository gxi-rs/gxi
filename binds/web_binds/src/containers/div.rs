use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::*;
use rust_gui_interface::{WeakNodeRc, NodeRc, Node};

pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct Div {
    pub parent: WeakNodeRc,
    pub dirty: bool,
    pub self_substitute: Option<WeakNodeRc>,
    pub child: Option<NodeRc>,
    pub sibling: Option<NodeRc>,
    pub widget: web_sys::HtmlElement,
}

impl Node for Div {
    impl_node_trait!();
    impl_node_trait_init_sibling!();
    impl_node_trait_init_child!();
    impl_node_trait_get_widget!();
    impl_node_trait_get_sibling!();
    impl_node_trait_get_child!();
    impl_node_trait_get_widget_as_container!();
    impl_node_trait_add!();
    impl_node_trait_substitute!();

    fn new(parent: WeakNodeRc) -> NodeRc {
        let this: NodeRc = Rc::new(RefCell::new(Box::new(Self {
            parent,
            dirty: true,
            self_substitute: None,
            child: None,
            sibling: None,
            widget: web_sys::document(),
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
            state.widget.show_all();
        }
        state.mark_clean();
    }
}

impl Div {}

impl_drop_for_node!(Div);
