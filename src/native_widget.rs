use crate::NativeWidget;

pub trait NativeContainerExt {
    fn append(&mut self, widget: &NativeWidget);
    fn move_to_index(&mut self, widget: &NativeWidget, index: usize);
}
