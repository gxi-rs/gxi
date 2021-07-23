use crate::{self as gxi, ContainerNode, WeakNodeType, WebContainerWrapper};

#[derive(crate::ContainerWidget)]
pub struct WebContainer {
    node: ContainerNode,
    native_widget: WebContainerWrapper,
}

impl WebContainer {
    pub fn from_str(from: &'static str, parent: WeakNodeType) -> Self {
        Self {
            node: ContainerNode {
                parent,
                child: Default::default(),
                sibling: Default::default(),
            },
            native_widget: WebContainerWrapper::from(from),
        }
    }
}
