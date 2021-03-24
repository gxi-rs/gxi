use crate::nodes::node::{AsyncNode, Node};
use gtk::ContainerExt;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Button {
    pub widget: gtk::Button,
    pub sibling: Option<AsyncNode>,
    pub parent: AsyncNode,
}

impl Node for Button {
    impl_node_trait!();
    impl_node_trait_init_sibling!();
    impl_node_trait_get_widget!();

    fn init_child(&mut self, _f: Box<dyn Fn() -> AsyncNode>, _add_widget: bool) -> (AsyncNode, bool) {
        panic!("Attempt to a add node into Button. Button can't have a child.");
    }
}

impl Button {
    pub fn new(parent: AsyncNode) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(Button {
            widget: gtk::Button::new(),
            sibling: None,
            parent,
        })))
    }
}
