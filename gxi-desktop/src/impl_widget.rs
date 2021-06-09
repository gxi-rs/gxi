use gtk::prelude::*;
use std::any::Any;

pub enum GtkElement {
    Widget(gtk::Widget),
    Container(gtk::Container)
}

impl gxi::Widget for GtkElement {
    fn append(&mut self, widget:&dyn gxi::Widget) {
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

