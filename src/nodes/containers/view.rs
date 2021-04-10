use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::{prelude::*};

use crate::nodes::node::{Node, NodeRc, NodeType};
use crate::WeakNodeRc;

pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct View {
    pub parent: WeakNodeRc,
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
    impl_node_trait_add!();

    fn new(parent: WeakNodeRc) -> NodeRc {
        Rc::new(RefCell::new(Box::new(View {
            parent,
            dirty: true,
            child: None,
            sibling: None,
            widget: gtk::Box::new(gtk::Orientation::Horizontal, 0),
        })))
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

impl View {
    pub fn orientation(&mut self, orientation: Orientation) {
        match orientation {
            Orientation::Horizontal => {
                if let gtk::Orientation::Vertical = self.widget.get_orientation() {
                    self.widget.set_orientation(gtk::Orientation::Horizontal);
                    self.mark_dirty();
                }
            }
            Orientation::Vertical => {
                if let gtk::Orientation::Horizontal = self.widget.get_orientation() {
                    self.widget.set_orientation(gtk::Orientation::Vertical);
                    self.mark_dirty();
                }
            }
        }
    }
}

impl_drop_for_node!(View);
