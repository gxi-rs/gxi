use crate::{create_widget, impl_widget};

create_widget!(Text, Label);

impl Node for Text {
    impl_widget!(Text, Label, (None));
}

impl Text {
    pub fn label(&self, label: &str) { self.widget.set_label(label); }
}
