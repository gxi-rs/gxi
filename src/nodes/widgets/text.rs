use crate::impl_widget;

impl_widget!(Text, Label, (None));

impl Text {
    pub fn label(&self, label: &str) {
        self.widget.set_label(label);
    }
}
