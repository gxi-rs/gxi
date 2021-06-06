use gtk::{Container, WidgetExt};
use gtk::prelude::*;

use gxi::NativeWidget;

use crate::glib::bitflags::_core::any::Any;

pub struct GtkContainer<T: ContainerExt + IsA<gtk::Container> + IsA<gtk::Widget>>(pub T);

impl<T: glib::IsA<gtk::Container> + IsA<gtk::Widget>> NativeWidget for GtkContainer<T> {
    fn append(&mut self, widget: &dyn NativeWidget) {
        let widget = widget.as_any().downcast_ref::<GtkContainer<Container>>().unwrap();
        self.0.add(&widget.0);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct GtkWidget<T: WidgetExt + IsA<gtk::Widget>>(pub T);

impl<T: WidgetExt + IsA<gtk::Widget>> NativeWidget for GtkWidget<T> {
    fn append(&mut self, _widget: &dyn NativeWidget) {
        panic!("can't add a child to this widget")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
