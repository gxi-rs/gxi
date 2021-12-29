use crate::{self as gxi, StrongNodeType, WebContainerWrapper};

#[derive(Default, crate::ContainerWidget)]
pub struct WebContainer {
    children: Vec<Option<StrongNodeType>>,
    native_widget: WebContainerWrapper,
}

impl<T: AsRef<str>> From<T> for WebContainer {
    fn from(from: T) -> Self {
        Self {
            children: Default::default(),
            native_widget: WebContainerWrapper::from(from),
        }
    }
}
