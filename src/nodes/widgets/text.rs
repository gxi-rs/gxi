use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;

use crate::nodes::node::{AsyncNode, Node, NodeType};

pub struct Text {
    pub widget: gtk::Label,
    pub sibling: Option<AsyncNode>,
}

impl Node for Text {
    impl_node_trait!();
    impl_node_trait_init_sibling!();
    impl_node_trait_get_widget!();
    impl_node_trait_get_sibling!();

    fn init_child(
        &mut self, _f: Box<dyn FnOnce() -> AsyncNode>, _parent: gtk::Container,
    ) -> (AsyncNode, bool) {
        panic!("Attempt to a add node into Text. Text can't have a child.");
    }

    fn new(_parent_widget: Option<gtk::Container>) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(Text {
            widget: gtk::Label::new(None),
            sibling: None,
        })))
    }

    fn get_widget_as_container(&self) -> gtk::Container {
        panic!("Text is not a container");
    }
}

impl Text {
    pub fn label(&self, label: &str) {
        self.widget.set_label(label);
    }
}

impl_drop_for_node!(Text);
