use crate::{self as gxi, WebContainerWrapper};

#[derive(Default, crate::Widget)]
pub struct WebWidget {
    node: crate::Node,
    native_widget: WebContainerWrapper,
}

impl From<WebContainerWrapper> for WebWidget {
    fn from(native_widget: WebContainerWrapper) -> Self {
        Self {
            node: Default::default(),
            native_widget,
        }
    }
}
