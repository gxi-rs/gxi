use std::any::Any;
use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;

use gtk::{ContainerExt, Orientation};

use crate::nodes::node::{AsyncNode, Node};

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
    /*
    fn get_widget(&self) -> Rc<gtk::Widget> {
        self.widget.try_into().unwrap()
    }

    fn get_widget_as_container(&self) -> Rc<gtk::Container> {
        self.widget.clone()
    }*/
}

impl View {
    pub fn new(parent: AsyncNode) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(View {
            child: None,
            sibling: None,
            parent,
            widget: gtk::Box::new(Orientation::Horizontal, 1),
        })))
    }
}
