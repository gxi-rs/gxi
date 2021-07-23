<<<<<<< Updated upstream
use std::{borrow::BorrowMut, cell::RefCell, ops::DerefMut, rc::Rc};

use crate::{StrongNodeType, VNodeType, WeakNodeType, VComponent};
=======
use std::{borrow::BorrowMut};

use crate::{VNodeType};
>>>>>>> Stashed changes

pub enum InitType {
    Child,
    Sibling,
}

<<<<<<< Updated upstream
pub fn init_member<C: FnOnce(WeakNodeType) -> VNodeType>(
    this: &StrongNodeType,
    init_type: InitType,
    init: C,
) -> Result<StrongNodeType, &'static str> {
    match init_type {
        InitType::Child => {
            let mut this_borrow = this.as_ref().borrow_mut();

            let node = match this_borrow.deref_mut() {
                VNodeType::Component(comp) => &mut comp.get_node_mut().child,
                VNodeType::Widget(_) => {
                    return Err("Can't add node to a widget. Use a container instead.")
                }
                VNodeType::ContainerWidget(cont) => &mut cont.get_node_mut().child,
                VNodeType::TopLevelContainerWidget(top) => &mut top.get_node_mut().child,
                VNodeType::TopLevelContainer(top) => &mut top.get_node_mut().child,
            };

            let child = node.get_or_insert_with(|| Rc::new(RefCell::new(init(Rc::downgrade(this)))));

            Ok(child.clone())
        }
        InitType::Sibling => {
            todo!()
=======
impl VNodeType {
    pub fn init_member<C: FnOnce() -> VNodeType>(
        &mut self,
        init_type: InitType,
        init: C,
    ) -> Result<&mut VNodeType, &'static str> {
        match init_type {
            InitType::Child => {
                let node = match self {
                    VNodeType::Widget(widget) => widget.get_node(),
                    VNodeType::TopLevelWidget(top) => top.get_node_ref().borrow_mut(),
                    VNodeType::Component(comp) => comp.get_node_ref().borrow_mut(),
                };
                Ok(node.get_child_mut()?.get_or_insert_with(init))
            }
            InitType::Sibling => {
                todo!()
            }
>>>>>>> Stashed changes
        }
    }
}
