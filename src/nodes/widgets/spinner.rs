use crate::{create_widget, impl_widget};

create_widget!(Spinner);

impl Node for Spinner {
    impl_widget!(Spinner);
}

impl Spinner {
    pub fn spin(&self, should_spin: bool) {
        if should_spin {
            self.widget.start()
        } else {
            self.widget.stop()
        }
    }
}
