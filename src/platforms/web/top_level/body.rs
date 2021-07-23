<<<<<<< Updated upstream
use crate::{TopLevelNode, WebContainerWrapper};
=======
use crate::{NativeWidget,NativeContainerExt, Node, VTopLevelWidget};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
>>>>>>> Stashed changes

use crate as gxi;

#[derive(gxi::TopLevelContainerWidget)]
pub struct Body {
<<<<<<< Updated upstream
    node: TopLevelNode,
    native_widget: WebContainerWrapper,
=======
    node: Rc<RefCell<Node>>,
    element: web_sys::HtmlElement,
}

impl VTopLevelWidget for Body {}

impl NativeContainerExt for Body {
    fn append(&mut self, _widget: &dyn NativeWidget) {
        todo!()
    }

    fn move_to_index(&mut self, _widget: &dyn NativeWidget, _index: usize) {
        todo!()
    }
>>>>>>> Stashed changes
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

