use std::any::Any;

use gtk::{Container, Widget};

pub use button::*;

use crate::{AsyncNode, Node};

pub mod button;
mod empty;

const PANIC_MSG: &str = "You can't call any function on (). () can only be used as an empty Node without any child or sibling";

impl Node for () {
    fn init_child(&mut self, _f: Box<dyn Fn() -> AsyncNode>, _add_widget: bool) -> (AsyncNode, bool) {
        panic!(PANIC_MSG)
    }

    fn init_sibling(&mut self, _f: Box<dyn Fn() -> AsyncNode>, _add_widget: bool) -> (AsyncNode, bool) {
        panic!(PANIC_MSG)
    }

    fn as_any(&self) -> &dyn Any {
        panic!(PANIC_MSG)
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        panic!(PANIC_MSG)
    }

    fn get_widget(&self) -> Widget {
        panic!(PANIC_MSG)
    }

    fn get_widget_as_container(&self) -> Container {
        panic!(PANIC_MSG)
    }

    fn get_parent(&self) -> AsyncNode {
        panic!(PANIC_MSG)
    }

    fn new(_parent: AsyncNode) -> AsyncNode {
        panic!(PANIC_MSG)
    }
}