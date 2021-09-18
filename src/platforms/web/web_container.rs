use crate::{self as gxi, TreeNode, WebContainerWrapper};

#[derive(Default, crate::ContainerWidget)]
pub struct WebContainer {
    node: TreeNode,
    native_widget: WebContainerWrapper,
}

impl WebContainer {
    pub fn from_str(from: &'static str) -> Self {
        Self {
            node: TreeNode::default(),
            native_widget: WebContainerWrapper::from(from),
        }
    }
}
