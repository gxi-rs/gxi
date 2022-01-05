use crate::{self as gxi, WebElement};

#[derive(gxi_derive::TopLevelContainerNode)]
pub struct Body {
    native_widget: WebElement,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            native_widget: WebElement::new({
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                document.body().unwrap().into()
            }),
        }
    }
}
