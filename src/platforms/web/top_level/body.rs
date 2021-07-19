use crate::{ContainerNode, WebContainerWrapper};

use crate as gxi;

#[derive(gxi::TopLevelContainerWidget)]
pub struct Body {
    node: ContainerNode,
    native_widget: WebContainerWrapper,
}

impl Default for Body {
    fn default() -> Self {
        Self {
            node: Default::default(),
            native_widget: WebContainerWrapper({
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                document.body().unwrap().into()
            }),
        }
    }
}














