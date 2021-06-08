use gtk::prelude::*;
use gtk::{Container, WidgetExt};
use gxi::Widget;

pub struct GtkContainer<T: ContainerExt + IsA<gtk::Container> + IsA<gtk::Widget>>(pub T);

impl<T: glib::IsA<gtk::Container> + IsA<gtk::Widget>> Widget for GtkContainer<T> {
    fn append(&mut self, widget: &dyn Widget) {
        let widget = widget
            .as_any()
            .downcast_ref::<GtkContainer<Container>>()
            .unwrap();
        self.0.add(&widget.0);
    }
}

pub struct GtkWidget<T: WidgetExt + IsA<gtk::Widget>>(pub T);

impl<T: WidgetExt + IsA<gtk::Widget>> Widget for GtkWidget<T> {
    fn append(&mut self, _widget: &dyn Widget) {
        panic!("can't add a child to this widget")
    }
}
