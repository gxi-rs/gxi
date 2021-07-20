use std::{cell::RefCell, rc::Rc};

pub fn run<C: crate::VNode + crate::Renderable + 'static>() {
    let root = Rc::new(RefCell::new(Root::default()))
    let node_rc = Rc::new(RefCell::new(C::new().into_vnode_type()));
    C::render(&node_rc);
}
