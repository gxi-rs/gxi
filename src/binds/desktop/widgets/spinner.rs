use crate::*;
use gtk::SpinnerExt;

create_widget!(Spinner);

impl_widget!(Spinner);

impl Spinner {
    pub fn spin(&self, should_spin: bool) {
        if should_spin {
            self.widget.start()
        } else {
            self.widget.stop()
        }
    }
}
