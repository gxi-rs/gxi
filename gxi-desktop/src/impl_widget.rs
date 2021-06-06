use gtk::{Container, WidgetExt};
use gtk::prelude::*;

use gxi::NativeWidget;

use crate::glib::bitflags::_core::any::Any;

pub struct GtkElement<T: ContainerExt + WidgetExt>(pub T);

impl<T: glib::IsA<gtk::Container> + IsA<gtk::Widget>> NativeWidget for GtkElement<T> {
    fn append(&mut self, widget: &dyn NativeWidget) {
        let widget = widget.as_any().downcast_ref::<GtkElement<Container>>().unwrap();
        self.0.add(&widget.0);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
