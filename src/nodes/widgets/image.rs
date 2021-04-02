use std::path::Path;

use crate::impl_widget;

impl_widget!(Image);

impl Image {
    pub fn source(&self, path: &str) {
        self.widget.set_from_file(&Path::new(path));
    }

    pub fn height(&self, height: i32) {
        self.widget.set_property_height_request(height);
    }

    pub fn width(&self, width: i32) {
        self.widget.set_property_width_request(width);
    }
}