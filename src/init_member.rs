use std::{cell::RefCell, ops::DerefMut, rc::Rc};

use crate::{StrongNodeType, VNodeType, WeakNodeType};

pub enum InitType {
    Child,
    Sibling,
}

/// @return ( child, is_child_new )
pub fn init_member<C: FnOnce(WeakNodeType) -> VNodeType>(
    this: &StrongNodeType,
    init_type: InitType,
    init: C,
) -> Result<(StrongNodeType, bool), &'static str> {
    match init_type {
        InitType::Child => {
            let mut this_borrow = this.as_ref().borrow_mut();


            let child = match this_borrow.deref_mut() {
                VNodeType::Component(comp) => &mut comp.get_node_mut().child,
                VNodeType::Widget(_) => {
                    return Err("Can't add node to a widget. Use a container instead.")
                }
                VNodeType::ContainerWidget(cont) => &mut cont.get_node_mut().child,
                VNodeType::TopLevelContainerWidget(top) => &mut top.get_node_mut().child,
                VNodeType::TopLevelContainer(top) => &mut top.get_node_mut().child,
            };
            
            // if child already exists return it
            if let Some(child) =  child {
                return Ok((child.to_owned(), true)); 
            }

            let child = child.get_or_insert_with(|| Rc::new(RefCell::new(init(Rc::downgrade(this)))));
            
            Ok((child.to_owned(), false))
        }
        InitType::Sibling => {
            todo!()
        }
    }
}
