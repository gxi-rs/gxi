use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::{Orientation, prelude::*};

use crate::nodes::node::{NodeRc, Node, NodeType};

pub struct View {
    pub dirty: bool,
    pub child: Option<NodeRc>,
    pub sibling: Option<NodeRc>,
    pub widget: gtk::Box,
}

impl Node for View {
    impl_node_trait!();
    impl_node_trait_init_sibling!();
    impl_node_trait_init_child!();
    impl_node_trait_get_widget!();
    impl_node_trait_get_sibling!();
    impl_node_trait_get_child!();
    impl_node_trait_get_widget_as_container!();

    fn new(_parent_widget: Option<gtk::Container>) -> NodeRc {
        Rc::new(RefCell::new(Box::new(View {
            dirty: true,
            child: None,
            sibling: None,
            widget: gtk::Box::new(Orientation::Horizontal, 1),
        })))
    }

    fn render(state: NodeRc) where Self: Sized {
        let mut state = state.as_ref().borrow_mut();
        let state = state.as_any_mut().downcast_mut::<Self>().unwrap();
        if state.dirty {
            state.widget.show_all();
        }
        state.mark_clean();
    }
}

impl_drop_for_node!(View);
