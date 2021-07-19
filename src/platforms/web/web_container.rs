use crate::{self as gxi, ContainerNode, WebContainerWrapper, WidgetNode};

#[derive(Default, crate::ContainerWidget)]
pub struct WebContainer {
    node: ContainerNode,
    native_widget: WebContainerWrapper,
}

impl From<WebContainerWrapper> for WebContainer {
    fn from(native_widget: WebContainerWrapper) -> Self {
        Self {
            node: Default::default(),
            native_widget,
        }
    }
}
