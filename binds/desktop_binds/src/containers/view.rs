use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use rust_gui_interface::{Node, NodeRc, WeakNodeRc};

use crate::*;

pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct View {
    pub parent: WeakNodeRc,
    pub dirty: bool,
    pub self_substitute: Option<WeakNodeRc>,
    pub child: Option<NodeRc>,
    pub sibling: Option<NodeRc>,
    pub widget: gtk::Box,
}

impl Node for View {
    impl_node_as_any!();
    impl_node_dirty!();
    impl_node_trait_init_sibling!();
    impl_node_trait_init_child!();
    impl_node_trait_get_widget!();
    impl_node_trait_get_sibling!();
    impl_node_trait_get_child!();
    impl_add_for_desktop_node!();
    impl_node_trait_substitute!();

    fn new(parent: WeakNodeRc) -> NodeRc {
        let this: NodeRc = Rc::new(RefCell::new(Box::new(Self {
            parent,
            dirty: true,
            self_substitute: None,
            child: None,
            sibling: None,
            widget: gtk::Box::new(gtk::Orientation::Horizontal, 0),
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

impl View {
    pub fn orientation(&mut self, orientation: Orientation) {
        match orientation {
            Orientation::Horizontal => {
                if let gtk::Orientation::Vertical = self.widget.get_orientation() {
                    self.widget.set_orientation(gtk::Orientation::Horizontal);
                }
            }
            Orientation::Vertical => {
                if let gtk::Orientation::Horizontal = self.widget.get_orientation() {
                    self.widget.set_orientation(gtk::Orientation::Vertical);
                }
            }
        }
    }

    pub fn h_expand(&mut self, h_expand: bool) {
        if h_expand != self.widget.get_hexpand() {
            self.widget.set_hexpand(true);
        }
    }

    pub fn v_expand(&mut self, v_expand: bool) {
        if v_expand != self.widget.get_vexpand() {
            self.widget.set_vexpand(true);
        }
    }
}

impl_drop_for_node!(View);
