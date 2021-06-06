use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gxi::*;

use crate::impl_drop;
use crate::impl_widget::GtkContainer;
use crate::*;

pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct View {
    parent: WeakGxiNodeType,
    child: Option<GxiNodeType>,
    sibling: Option<GxiNodeType>,
    widget: GtkContainer<gtk::Box>,
}

impl GxiNode for View {
    impl_node_trait_as_any!();
    impl_node_member_getters!();

    fn new(parent: WeakGxiNodeType) -> GxiNodeType {
        GxiNodeType::Widget(Rc::new(RefCell::new(Box::new(Self {
            parent,
            child: None,
            sibling: None,
            widget: GtkContainer(gtk::Box::new(gtk::Orientation::Horizontal, 0)),
        }))))
    }
}

impl_widget_node!(View);

impl View {
    pub fn orientation(&mut self, orientation: Orientation) {
        match orientation {
            Orientation::Horizontal => {
                if let gtk::Orientation::Vertical = self.widget.0.get_orientation() {
                    self.widget.0.set_orientation(gtk::Orientation::Horizontal);
                }
            }
            Orientation::Vertical => {
                if let gtk::Orientation::Horizontal = self.widget.0.get_orientation() {
                    self.widget.0.set_orientation(gtk::Orientation::Vertical);
                }
            }
        }
    }

    pub fn h_expand(&mut self, h_expand: bool) {
        if h_expand != self.widget.0.get_hexpand() {
            self.widget.0.set_hexpand(true);
        }
    }

    pub fn v_expand(&mut self, v_expand: bool) {
        if v_expand != self.widget.0.get_vexpand() {
            self.widget.0.set_vexpand(true);
        }
    }
}

impl_drop!(View);
