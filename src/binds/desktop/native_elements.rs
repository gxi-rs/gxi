use gtk::prelude::*;
use std::any::Any;

pub struct GtkWidget(pub Box<dyn AsRef<gtk::Widget>>);

impl gxi::NativeWidget for GtkWidget {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct GtkContainer(pub Box<dyn AsRef<gtk::Container>>);

impl gxi::NativeContainer for GtkContainer {
    fn append(&mut self, widget: &dyn gxi::NativeWidget) {
        self.0.as_ref().add(&widget.as_any().downcast_ref::<GtkWidget>().unwrap().0)
    }
}

impl gxi::NativeWidget for GtkContainer {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
/*

pub enum GtkElement {
    Widget(gtk::Widget),
    Container(gtk::Container),
}

impl gxi::Widget for GtkElement {
    fn append(&mut self, widget: &dyn gxi::Widget) {
        let widget = widget.as_any().downcast_ref::<GtkElement>().unwrap();

        match self {
            GtkElement::Container(this) => match widget {
                GtkElement::Container(widget) => this.add(widget),
                GtkElement::Widget(widget) => this.add(widget)
            },
            _ => panic!(""),
        }
    }
    gxi::impl_node_trait_as_any!();
}

*/