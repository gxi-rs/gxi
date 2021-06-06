use std::path::Path;

use gxi::*;
use crate::*;

create_widget!(Image);

impl_widget!(Image);

impl Image {
    pub fn source(&self, path: &str) {
        self.widget.0.set_from_file(&Path::new(path));
    }

    pub fn height(&self, height: i32) {
        self.widget.0.set_property_height_request(height);
    }

    pub fn width(&self, width: i32) {
        self.widget.0.set_property_width_request(width);
    }
}
