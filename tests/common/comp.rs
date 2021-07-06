use std::cell::RefCell;
use std::rc::Rc;

use gxi::{GxiComponent, Node};

/// App is a component which is cloneable
#[derive(Clone, Default, GxiComponent)]
pub struct Comp {
    node: Rc<RefCell<Node>>,
}
