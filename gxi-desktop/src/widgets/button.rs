use gxi::*;
use crate::*;

create_widget!(Button);

impl_widget!(Button);

impl Button {
    pub fn label(&self, label: &str) {
        self.widget.0.set_label(label);
    }

    pub fn on_click<F: Fn() + 'static>(&self, f: F) {
        self.widget.0.connect_clicked(move |_| f());
    }
}
