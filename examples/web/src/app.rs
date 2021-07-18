use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use gxi::VComponent;
use gxi::InitType;
use gxi::VNode;

#[derive(Clone, Default, gxi::Component)]
pub struct App {
    node: Rc<RefCell<gxi::Node>>,
}

impl gxi::Renderable for App {
    fn render(&mut self) {
        let mut node_ref = self.node.as_ref().borrow_mut();
        
        let node = self.into_vnode_type().init_member( InitType::Child, || gxi::Body::default() ).unwrap();
        
        let _node = node.init_member(&mut node.get_node_ref().borrow_mut(), InitType::Child, || {
                gxi::WebElement::from("h1").into()
            }).unwrap();
    }
}
