use crate::{self as gxi, gxi_derive::gxi_vnode};

#[gxi_vnode(Leaf)]
pub struct Text {
    native_widget: web_sys::Text,
}

impl Default for Text {
    fn default() -> Self {
        Self::from("")
    }
}

impl gxi::VLeaf for Text {}

impl std::ops::Deref for Text {
    type Target = gxi::NativeWidget;

    fn deref(&self) -> &Self::Target {
        &self.native_widget
    }
}

impl std::ops::DerefMut for Text {
    fn deref_mut(&mut self) -> &mut Self::Target {
        panic!("cannot borrow text node as mut")
    }
}

impl<T: AsRef<str>> From<T> for Text {
    fn from(value: T) -> Self {
        Self {
            native_widget: {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                document.create_text_node(value.as_ref())
            },
        }
    }
}

impl Text {
    pub fn value<T: AsRef<str>>(&mut self, value: T) {
        self.native_widget.set_text_content(Some(value.as_ref()));
    }
}
