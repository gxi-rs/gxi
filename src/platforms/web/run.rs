use std::{cell::RefCell, rc::Rc};

use crate::{Root, VNode};

pub fn run<C: crate::VNode + crate::Renderable + 'static>() {
    let root = Rc::new(RefCell::new(Root::default().into_vnode_type()));
    let node_rc = Rc::new(RefCell::new(C::new(Rc::downgrade(&root)).into_vnode_type()));
    C::render(&node_rc);
}
