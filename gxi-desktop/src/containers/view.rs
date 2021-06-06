use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gxi::*;

use crate::*;
use crate::impl_drop;
use crate::impl_widget::GtkElement;

pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct View {
    parent: WeakNodeType,
    child: Option<NodeType>,
    sibling: Option<NodeType>,
    widget: GtkElement<gtk::Box>,
}

impl Node for View {
    impl_node_trait_as_any!();
    //impl_node_trait_get_widget!();
    //impl_node_trait_get_sibling!();
    //impl_node_trait_get_child!();
    //impl_node_trait_get_parent!();
    //impl_add_for_desktop_node!();
    //impl_node_trait_substitute!();

    fn new(parent: WeakNodeType) -> NodeType {
        NodeType::Widget(Rc::new(RefCell::new(Box::new(Self {
            parent,
            child: None,
            sibling: None,
            widget: GtkElement(gtk::Box::new(gtk::Orientation::Horizontal, 0)),
        }))))
    }

    impl_node_member_getters!();
}

impl WidgetNode for View {
    fn get_widget(&self) -> &dyn NativeWidget {
        &self.widget
    }
    fn get_widget_mut(&mut self) -> &mut dyn NativeWidget {
        &mut self.widget
    }
}

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
