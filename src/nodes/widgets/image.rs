use std::path::Path;

use crate::impl_widget;

impl_widget!(Image);

impl Image {
    pub fn source(&self, path: &Path) {
        self.widget.set_from_file(path);
    }
}