use std::cell::RefCell;
use std::rc::Rc;

use crate::{AsyncNode, Node};

pub trait Component: Node {
    fn new(parent: AsyncNode) -> Rc<RefCell<Self>>;
    fn render(container: AsyncNode, top_state: Rc<RefCell<Self>>);
}