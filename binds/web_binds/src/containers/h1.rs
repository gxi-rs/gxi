use crate::*;

create_web_container!(H1);

impl_web_container!(H1 "h1");

impl H1 {
    pub fn label(&self, text: &str) {
        self.widget.set_inner_html(&text);
    }
}
