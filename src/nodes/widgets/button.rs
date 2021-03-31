use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;

use crate::nodes::node::{AsyncNode, Node, NodeType};

pub struct Button {
    pub widget: gtk::Button,
    pub sibling: Option<AsyncNode>,
}

impl Node for Button {
    impl_node_trait!();
    impl_node_trait_init_sibling!();
    impl_node_trait_get_widget!();
    impl_node_trait_get_sibling!();

    fn init_child(
        &mut self, _f: Box<dyn FnOnce() -> AsyncNode>, _parent: gtk::Container,
    ) -> (AsyncNode, bool) {
        panic!("Attempt to a add node into Button. Button can't have a child.");
    }

    fn get_widget_as_container(&self) -> gtk::Container {
        panic!("Text is not a container");
    }

    fn new(_parent_widget: Option<gtk::Container>) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(Button {
            widget: gtk::Button::new(),
            sibling: None,
        })))
    }
}

impl Button {
    pub fn label(&self, label: &str) {
        self.widget.set_label(label);
    }

    pub fn on_click<F: Fn(&gtk::Button) + 'static>(&self, f: F) {
        self.widget.connect_clicked(f);
    }
}

impl_drop_for_node!(Button);
