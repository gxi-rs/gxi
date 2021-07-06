use crate::web_element::WebElement;

#[derive(Default, gxi::Widget)]
pub struct WebWidget {
    node: gxi::Node,
    element: WebElement,
}

impl From<WebElement> for WebWidget {
    fn from(element: WebElement) -> Self {
        WebWidget {
            node: Default::default(),
            element,
        }
    }
}
