use std::ops::{Deref, DerefMut};

use crate::{NativeWidget, NativeContainerExt};

pub struct WebContainerWrapper(pub web_sys::Element);

impl NativeContainerExt for WebContainerWrapper {
    fn append(&mut self, _widget: &NativeWidget) {
        todo!()
    }

    fn move_to_index(&mut self, _widget: &NativeWidget, _index: usize) {
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
