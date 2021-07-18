use std::ops::DerefMut;

use crate::{Node, VNode, VNodeType};

pub enum InitType {
    Child,
    Sibling,
}

impl VNodeType {
    pub fn init_member<C: FnOnce() -> VNode>(
        &mut self,
        init_type: InitType,
        init: C,
    ) -> Result<&mut VNodeType, &'static str> {
        match init_type {
            InitType::Child => {
                let child = self.get_child_mut()?;
                child.get_or_insert_with(|| init().into_vnode_type())
            }
            InitType::Sibling => {
                let sibling = match self {
                    Node::Widget(c) => &mut c.sibling,
                    Node::Container(c) => &mut c.sibling,
                };

                let t = sibling.get_or_insert_with(|| init().into_vnode_type());
                Ok(t)
            }
        }
    }
}
