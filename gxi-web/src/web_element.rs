use gxi::NativeWidget;

pub struct WebElement(web_sys::Element);

impl NativeWidget for WebElement {}

impl Default for WebElement {
    fn default() -> Self {
        "div".into()
    }
}

impl From<&str> for WebElement {
    fn from(name: &str) -> Self {
        Self({
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            document.create_element(name).unwrap()
        })
    }
}
