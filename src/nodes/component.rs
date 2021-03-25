use std::cell::RefCell;
use std::rc::Rc;

use crate::AsyncNode;

pub trait Component: Default {
    fn render(container: AsyncNode, top_state: Rc<RefCell<Self>>);
}