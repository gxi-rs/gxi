create_widget!(H1);

impl_widget!(H1 {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    document.create_element("h1").unwrap()
});

impl H1 {
    pub fn label(&self, text: &str) {
        self.widget.set_inner_html(&text);
    }
}

