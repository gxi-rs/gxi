use std::ops::{Deref, DerefMut};

use crate::web_element::WebElement;

#[derive(Default, gxi::Widget)]
pub struct WebWidget {
    node: gxi::Node,
    element: WebElement,
}

impl Deref for WebWidget {
    type Target = WebElement;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

impl DerefMut for WebWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.element
    }
}

impl From<WebElement> for WebWidget {
    fn from(element: WebElement) -> Self {
        WebWidget {
            node: Default::default(),
            element,
        }
    }
}
