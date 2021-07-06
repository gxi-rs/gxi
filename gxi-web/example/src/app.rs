use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Default, gxi::Component)]
pub struct App {
    node: Rc<RefCell<gxi::Node>>,
}

impl gxi::Renderable for App {}
