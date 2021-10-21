use crate::{self as gxi, StrongNodeType, WebContainerWrapper};

#[derive(gxi::TopLevelContainerWidget)]
pub struct Head {
    children: Vec<StrongNodeType>,
    native_widget: WebContainerWrapper,
}

impl Default for Head {
    fn default() -> Self {
        Self {
            children: Default::default(),
            native_widget: WebContainerWrapper({
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                document.head().unwrap().into()
            }),
        }
    }
}
