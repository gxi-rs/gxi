use crate::*;
use gxi::*;

create_widget!(Text, Label);

impl_widget!(Text, Label, (None));

impl Text {
    pub fn label(&self, label: &str) {
        self.widget.0.set_label(label);
    }
}
