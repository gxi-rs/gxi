use crate::nodes::node::{*};

impl_widget!(Button);

impl Button {
    pub fn label(&self, label: &str) {
        self.widget.set_label(label);
    }

    pub fn on_click<F: Fn(&gtk::Button) + 'static>(&self, f: F) {
        self.widget.connect_clicked(f);
    }
}


