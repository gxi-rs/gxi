use std::cell::RefCell;
use std::rc::Rc;

use gxi::Node;

#[derive(Clone, Default, gxi::Component)]
pub struct App {
    node: Rc<RefCell<Node>>,
}
