use std::{borrow::BorrowMut, cell::RefCell, ops::DerefMut, rc::Rc};

use crate::{StrongNodeType, VNodeType, WeakNodeType, VComponent};

pub enum InitType {
    Child,
    Sibling,
}

pub fn init_member<C: FnOnce(WeakNodeType) -> VNodeType>(
    this: &StrongNodeType,
    init_type: InitType,
    init: C,
) -> Result<StrongNodeType, &'static str> {
    match init_type {
        InitType::Child => {
            let mut this_borrow = this.as_ref().borrow_mut();

            let node = match this_borrow.deref_mut() {
                VNodeType::Component(comp) => comp.get_node_mut(),
                VNodeType::Widget(_) => {
                    return Err("Can't add node to a widget. Use a container instead.")
                }
                VNodeType::ContainerWidget(cont) => cont.get_node_mut(),
                VNodeType::TopLevelContainerWidget(top) => top.get_node_mut(),
            };

            let child = node
                .child
                .get_or_insert_with(|| Rc::new(RefCell::new(init(Rc::downgrade(this)))));

            Ok(child.clone())
        }
        InitType::Sibling => {
            todo!()
        }
    }
}
