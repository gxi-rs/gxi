use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

use gxi::InitType;

#[derive(Clone, Default, gxi::Component)]
pub struct App {
    node: Rc<RefCell<gxi::Node>>,
}

impl gxi::Renderable for App {
    fn render(&mut self) {
        let mut node_ref = self.node.as_ref().borrow_mut();
        let _node: &mut gxi::WebWidget =
            gxi::init_member(node_ref.deref_mut(), InitType::Child, || {
                gxi::WebElement::from("h1").into()
            });
    }
}
