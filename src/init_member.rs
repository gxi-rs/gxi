use std::ops::DerefMut;

use crate::{Node, VNode};

pub enum InitType {
    Child,
    Sibling,
}

pub fn init_member<C: FnOnce() -> N, N: VNode>(tree: &mut Node, init_type: InitType, init: C) -> &mut N {
    match init_type {
        InitType::Child => {
            let t = tree.child.get_or_insert_with(|| init().into_vnode_type());
            t.deref_mut().as_mut().downcast_mut().unwrap()
        }
        InitType::Sibling => {
            let t = tree.sibling.get_or_insert_with(|| init().into_vnode_type());
            t.deref_mut().as_mut().downcast_mut().unwrap()
        }
    }
}
