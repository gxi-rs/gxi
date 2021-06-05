use crate::*;

create_web_container!(A);

impl_web_container!(A "a");

impl A {
    pub fn label(&self, text: &str) {
        self.widget.set_inner_html(&text);
    }
    pub fn href(&self, text: &str) {
        self.widget.set_attribute("href", text).unwrap();
    }
}
