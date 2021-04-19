use crate::*;

create_web_container!(Button);

impl_web_container!(Button "button");

impl Button {
    pub fn label(&self, text: &str) {
        self.widget.set_inner_html(&text);
    }
}
