use std::cell::RefCell;
use std::rc::Rc;

use gxi::Node;

/// App is a component which is cloneable
#[derive(Clone, Default, gxi::Component)]
pub struct Comp {
    node: Rc<RefCell<Node>>,
}
