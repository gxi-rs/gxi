use std::{cell::RefCell, rc::Rc};

use crate::{init_member, InitType, Root, VNode};

pub fn run<C: crate::VNode + crate::Renderable + 'static>() {
    let root = Rc::new(RefCell::new(Root::default().into_vnode_type()));
    let (node, _) = init_member(&root, InitType::Child(false), |parent| {
        C::new(parent).into_vnode_type()
    })
    .unwrap();
    C::render(&node);
    std::mem::forget(root);
}
