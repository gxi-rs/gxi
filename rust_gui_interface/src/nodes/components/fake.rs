use std::any::Any;

use crate::*;

const PANIC_MSG: &str = "You can't call any function on Fake. Fake Widget can only be used as an empty Node without any child or sibling";

pub struct Fake;

impl Node for Fake {
    type NativeWidget = ();
    type NativeWidgetContainer = ();

    fn init_child(&mut self, _f: Box<dyn FnOnce() -> NodeRc<Self>>) -> (NodeRc<Self>, bool) {
        panic!("{}", PANIC_MSG);
    }

    fn init_sibling(&mut self, _f: Box<dyn FnOnce() -> NodeRc<Self>>) -> (NodeRc<Self>, bool) {
        panic!("{}", PANIC_MSG);
    }

    fn as_any(&self) -> &dyn Any {
        panic!("{}", PANIC_MSG);
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        panic!("{}", PANIC_MSG);
    }

    fn get_widget(&self) -> Self::NativeWidget {
        panic!("{}", PANIC_MSG);
    }

    fn get_widget_as_container(&self) -> Self::NativeWidgetContainer {
        panic!("{}", PANIC_MSG);
    }

    fn new(parent: WeakNodeRc<Self>) -> NodeRc<Self> where Self: Sized {
        panic!("{}", PANIC_MSG);
    }

    fn is_dirty(&self) -> bool {
        panic!("{}", PANIC_MSG);
    }

    fn mark_dirty(&mut self) {
        panic!("{}", PANIC_MSG);
    }

    fn mark_clean(&mut self) {
        panic!("{}", PANIC_MSG);
    }

    fn get_self_substitute(&self) -> NodeRc<Self> {
        panic!("{}", PANIC_MSG);
    }

    fn set_self_substitute(&mut self, self_substitute: NodeRc<Self>) {
        panic!("{}", PANIC_MSG);
    }

    fn add(&mut self, child: NodeRc<Self>) {
        panic!("{}", PANIC_MSG);
    }
}

impl Drop for Fake {
    fn drop(&mut self) {
        // Components need to not drop anything
    }
}
