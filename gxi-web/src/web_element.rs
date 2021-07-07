use std::ops::{Deref, DerefMut};

use gxi::NativeWidget;

pub struct WebElement(web_sys::Element);

impl NativeWidget for WebElement {
    fn append(&mut self, widget: &dyn NativeWidget) {
        todo!()
    }

    fn move_to_index(&mut self, widget: &dyn NativeWidget, index: usize) {
        todo!()
    }
}

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

impl Deref for WebElement {
    type Target = web_sys::Element;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WebElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
