use crate::{self as gxi, WebElement};

#[derive(Default, gxi_derive::ContainerNode)]
pub struct WebContainer {
    native_widget: WebElement,
}

impl<T: AsRef<str>> From<T> for WebContainer {
    fn from(from: T) -> Self {
        Self {
            native_widget: WebElement::from(from),
        }
    }
}
