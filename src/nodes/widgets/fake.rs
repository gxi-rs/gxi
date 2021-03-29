use std::any::Any;

use crate::nodes::node::{AsyncNode, Node, NodeType};
use gtk::{Container, Widget};

const PANIC_MSG: &str = "You can't call any function on (). () can only be used as an empty Node without any child or sibling";

pub struct Fake;

impl Node for Fake {
    fn init_child(&mut self, _f: Box<dyn FnOnce() -> AsyncNode>, _parent: AsyncNode) -> (AsyncNode, bool) {
        panic!("{}", PANIC_MSG);
    }

    fn init_sibling(&mut self, _f: Box<dyn FnOnce() -> AsyncNode>, parent: AsyncNode) -> (AsyncNode, bool) {
        panic!("{}", PANIC_MSG);
    }

    fn as_any(&self) -> &dyn Any {
        panic!("{}", PANIC_MSG);
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        panic!("{}", PANIC_MSG);
    }

    fn get_widget(&self) -> Widget {
        panic!("{}", PANIC_MSG);
    }

    fn get_widget_as_container(&self) -> Container {
        panic!("{}", PANIC_MSG);
    }

    fn new(parent_widget: Option<gtk::Container>) -> AsyncNode {
        panic!("{}", PANIC_MSG);
    }
}

impl_drop_for_component!(Fake);