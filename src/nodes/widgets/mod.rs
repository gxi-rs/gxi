pub use button::*;
use crate::{AsyncNode, Node};
use std::cell::RefCell;
use std::rc::Rc;
use gtk::{Container, Widget};
use std::any::Any;

pub mod button;
mod empty;

impl Node for () {
    fn init_child(&mut self, _f: Box<dyn Fn() -> AsyncNode>, _add_widget: bool) -> (AsyncNode, bool) {
        unimplemented!()
    }

    fn init_sibling(&mut self, _f: Box<dyn Fn() -> AsyncNode>, _add_widget: bool) -> (AsyncNode, bool) {
        unimplemented!()
    }

    fn as_any(&self) -> &dyn Any {
        unimplemented!()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        unimplemented!()
    }

    fn get_widget(&self) -> Widget {
        unimplemented!()
    }

    fn get_widget_as_container(&self) -> Container {
        unimplemented!()
    }

    fn get_parent(&self) -> AsyncNode {
        unimplemented!()
    }

    fn new(parent: AsyncNode) -> AsyncNode {
        unimplemented!();
    }
}