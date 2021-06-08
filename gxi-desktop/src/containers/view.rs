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
    parent: WeakNodeType,
    self_substitute: Option<WeakNodeType>,
    child: Option<StrongNodeType>,
    sibling: Option<StrongNodeType>,
    widget: GtkContainer<gtk::Box>,
}

impl Node for View {
    impl_node_trait_as_any!();
    impl_node_getters!();

    fn new(parent: WeakNodeType) -> StrongNodeType {
        Rc::new(RefCell::new(GxiNodeType::Container(Box::new(Self {
            parent,
            self_substitute: None,
            child: None,
            sibling: None,
            widget: GtkContainer(gtk::Box::new(gtk::Orientation::Horizontal, 0)),
        }))))
    }
}

impl ContainerNode for View {}
impl_container!(View);
impl_widget_node!(View);
impl_component_node!(View);

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
