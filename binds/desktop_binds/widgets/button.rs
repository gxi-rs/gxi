use crate::*;

create_widget!(Button);

impl Node for Button {
    impl_widget!(Button);
}

impl Button {
    pub fn label(&self, label: &str) {
        self.widget.set_label(label);
    }

    pub fn on_click<F: Fn() + 'static>(&self, f: F) {
        self.widget.connect_clicked(move |_| f());
    }
}
