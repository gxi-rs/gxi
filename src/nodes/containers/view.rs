use std::any::Any;
use std::cell::RefCell;

use std::rc::Rc;

use gtk::{prelude::*, Orientation, Container};

use crate::nodes::node::{AsyncNode, Node, NodeType};

pub struct View {
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub parent: AsyncNode,
    pub widget: gtk::Box,
}

impl Node for View {
    impl_node_trait!();
    impl_node_trait_init_sibling!();
    impl_node_trait_init_child!();
    impl_node_trait_get_widget!();
    impl_node_trait_get_sibling!();
    impl_node_trait_get_child!();

    fn new(parent: AsyncNode, _widget: Option<gtk::Container>) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(View {
            child: None,
            sibling: None,
            parent,
            widget: gtk::Box::new(Orientation::Horizontal, 1),
        })))
    }
}

impl Drop for View {
    fn drop(&mut self) {
        let parent_borrow =  self.parent.as_ref().borrow_mut();
        parent_borrow.get_widget_as_container().remove(&self.widget);
    }
}