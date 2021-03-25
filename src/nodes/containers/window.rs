use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::{ContainerExt, WindowType};

use crate::nodes::node::{AsyncNode, Node};

pub struct Window {
    pub child: Option<AsyncNode>,
    pub widget: gtk::Window,
}

impl Node for Window {
    impl_node_trait!();
    impl_node_trait_get_widget!();
    impl_node_trait_init_child!();

    fn init_sibling(
        &mut self, _f: Box<dyn Fn() -> AsyncNode>, _add_widget: bool,
    ) -> (AsyncNode, bool) {
        panic!("Window can't have a sibling node");
    }

    fn new(_parent: AsyncNode) -> AsyncNode {
        unimplemented!();
    }

}

impl Window {
    pub fn fake_new() -> AsyncNode{
        Rc::new(RefCell::new(Box::new(Window {
            child: None,
            widget: gtk::Window::new(WindowType::Toplevel),
        })))
    }
}

