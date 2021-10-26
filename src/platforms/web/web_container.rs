use crate::{self as gxi, StrongNodeType, WebContainerWrapper};

#[derive(Default, crate::ContainerWidget)]
pub struct WebContainer {
    children: Vec<StrongNodeType>,
    native_widget: WebContainerWrapper,
}

impl WebContainer {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(from: &'static str) -> Self {
        Self {
            children: Default::default(),
            native_widget: WebContainerWrapper::from(from),
        }
    }
}
