use std::{
    borrow::BorrowMut,
    ops::{Deref, DerefMut},
};

use crate::VNodeType;

pub enum InitType {
    Child,
    Sibling,
}

impl VNodeType {
    pub fn init_member<C: FnOnce() -> VNodeType>(
        &mut self,
        init_type: InitType,
        init: C,
    ) -> Result<&mut VNodeType, &'static str> {
        match init_type {
            InitType::Child => {
                let node = match self {
                    VNodeType::Component(comp) => comp.get_node_ref().get_mut(),
                    VNodeType::Widget(_) => {
                        return Err("Can't add node to a widget. Use a container instead.")
                    }
                    VNodeType::ContainerWidget(cont) => cont.get_node_mut(),
                    VNodeType::TopLevelContainerWidget(top) => top.get_node_mut(),
                };
                Ok(node.child.get_or_insert_with(init))
            }
            InitType::Sibling => {
                todo!()
            }
        }
    }
}
