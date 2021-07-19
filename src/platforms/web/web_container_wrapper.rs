use std::ops::{Deref, DerefMut};

use crate::{NativeContainer, NativeWidget, NativeContainerExt};

pub struct WebContainerWrapper(web_sys::Element);

impl NativeContainerExt for WebContainerWrapper {
    fn append(&mut self, widget: &NativeWidget) {
        todo!()
    }

    fn move_to_index(&mut self, widget: &NativeWidget, index: usize) {
        todo!()
    }
}

impl Default for WebContainerWrapper {
    fn default() -> Self {
        "div".into()
    }
}

impl From<&str> for WebContainerWrapper {
    fn from(name: &str) -> Self {
        Self({
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            document.create_element(name).unwrap()
        })
    }
}

impl Deref for WebContainerWrapper {
    type Target = web_sys::Element;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for WebContainerWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
