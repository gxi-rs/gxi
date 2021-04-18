use crate::*;

create_web_container!(El);

impl_web_container!(El "h1");

impl El {
    pub fn label(&self, text: &str) {
        self.widget.set_inner_html(&text);
    }
}