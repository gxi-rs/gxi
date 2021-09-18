use std::{cell::RefCell, rc::Rc};

pub fn run<C: crate::VNode + crate::Renderable + 'static>() {
    let node = Rc::new(RefCell::new(C::new().into_vnode_type()));
    C::render(&node);
    std::mem::forget(node);
}
