use crate::*;
use gxi::*;

create_widget!(Spinner);

impl_widget!(Spinner);

impl Spinner {
    pub fn spin(&self, should_spin: bool) {
        if should_spin {
            self.widget.0.start()
        } else {
            self.widget.0.stop()
        }
    }
}
