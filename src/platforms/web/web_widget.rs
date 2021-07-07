use std::ops::{Deref, DerefMut};

use crate as gxi;
use crate::WebElement;

#[derive(Default, crate::Widget)]
pub struct WebWidget {
    node: crate::Node,
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
