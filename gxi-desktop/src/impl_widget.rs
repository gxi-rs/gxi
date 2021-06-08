use gtk::prelude::*;
use gtk::{Container, WidgetExt};
use gxi::*;
use std::any::Any;

pub struct GtkContainer<T: ContainerExt + IsA<gtk::Container> + IsA<gtk::Widget>>(pub T);

impl<T: glib::IsA<gtk::Container> + IsA<gtk::Widget>> Widget for GtkContainer<T> {
    fn append(&mut self, widget: &dyn Widget) {
     //   return;
        let widget = widget
            .as_any()
            .downcast_ref::<GtkWidget<gtk::Widget>>()
            .unwrap();
        self.0.add(&widget.0);
    }

    impl_node_trait_as_any!();
}

pub struct GtkWidget<T: WidgetExt + IsA<gtk::Widget>>(pub T);

impl<T: WidgetExt + IsA<gtk::Widget>> Widget for GtkWidget<T> {
    fn append(&mut self, _widget: &dyn Widget) {
        panic!("can't add a child to this widget")
    }
    
    impl_node_trait_as_any!();
}
