use gxi::NativeWidget;
use gtk::{WidgetExt, ContainerExt};
use crate::glib::bitflags::_core::any::Any;

pub struct GtkContainer<T: ContainerExt>(T);

impl<T> NativeWidget for GtkContainer<T> {
    fn append(&mut self, widget: &dyn NativeWidget) {
        let widget = widget.as_any().downcast_ref::<dyn ContainerExt>().unwrap();
        self.0.add(widget);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
