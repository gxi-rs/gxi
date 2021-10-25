use crate::NativeContainerExt;

pub type NativeWidget = ();
pub type NativeContainerWidget = ();
pub type Element = ();

const PANIC_MSG:&str ="Can't construct a tree with default features. Consider enabling other features. eg. web, desktop";

impl NativeContainerExt for NativeContainerWidget {
    fn append(&mut self, _widget: &NativeWidget) {
        panic!("{}", PANIC_MSG);
    }
    fn move_to_index(&mut self, _widget: &NativeWidget, _index: usize) {
        panic!("{}", PANIC_MSG);
    }
}
