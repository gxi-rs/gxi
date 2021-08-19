use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::{
    native_widget::NativeContainerExt, NativeContainerWidget, NativeWidget, StrongNodeType,
    VNodeType, WeakNodeType,
};

pub enum InitType<'a> {
    Child,
    /// # Args
    /// reference to parent
    Sibling(&'a StrongNodeType),
}

/// # Args
///
/// * `this` - In case of InitType::Child, `this` is the parent and owner of the child node
///          - In case of InitType::Sibling, `this` is the owner but not the parent of the sibling node.
///
/// # Return
///
/// `( member, is_member_new )`
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
                    return Err("Can't add node to a widget. Use a container instead.");
                }
                VNodeType::ContainerWidget(cont) => &mut cont.get_node_mut().child,
                VNodeType::TopLevelContainerWidget(top) => &mut top.get_node_mut().child,
                VNodeType::TopLevelContainer(top) => &mut top.get_node_mut().child,
            };

            // if child already exists return it
            if let Some(child) = child {
                return Ok((child.to_owned(), false));
            }

            let child = child
                .get_or_insert_with(|| Rc::new(RefCell::new(init(Rc::downgrade(this)))))
                .to_owned();

            append_native_widget(this_borrow.deref_mut(), &child.borrow().deref())?;

            Ok((child, true))
        }
        InitType::Sibling(parent) => {
            let mut this_borrow = this.as_ref().borrow_mut();

            let sibling = match this_borrow.deref_mut() {
                VNodeType::Component(comp) => &mut comp.get_node_mut().sibling,
                VNodeType::Widget(w) => &mut w.get_node_mut().sibling,
                VNodeType::ContainerWidget(cont) => &mut cont.get_node_mut().sibling,
                VNodeType::TopLevelContainerWidget(top) => &mut top.get_node_mut().sibling,
                VNodeType::TopLevelContainer(top) => &mut top.get_node_mut().sibling,
            };

            // if child already exists return it
            if let Some(sibling) = sibling {
                return Ok((sibling.to_owned(), false));
            }

            let sibling = sibling
                .get_or_insert_with(|| Rc::new(RefCell::new(init(Rc::downgrade(parent)))))
                .to_owned();

            append_native_widget(parent.borrow_mut().deref_mut(), &sibling.borrow().deref())?;

            Ok((sibling, true))
        }
    }
}

fn append_native_widget(this: &mut VNodeType, member: &VNodeType) -> Result<(), &'static str> {
    // get native widget from member
    let widget: &NativeWidget = match member {
        VNodeType::Widget(w) => w.deref(),
        VNodeType::ContainerWidget(w) => w.deref(),
        _ => {
            return Ok(());
        }
    };

    let container_widget: &mut NativeContainerWidget = match this {
        VNodeType::TopLevelContainerWidget(t) => t.deref_mut(),
        VNodeType::ContainerWidget(t) => t.deref_mut(),
        VNodeType::Component(comp) => {
            // search for a widget up the tree
            let mut parent = comp.get_node_mut().parent.upgrade().unwrap();
            loop {
                let mut parent_borrow = parent.as_ref().borrow_mut();
                let parent_widget: &mut NativeContainerWidget = match parent_borrow.deref_mut() {
                    VNodeType::TopLevelContainerWidget(t) => t.deref_mut(),
                    VNodeType::ContainerWidget(c) => c.deref_mut(),
                    VNodeType::Component(c) => {
                        let pa = c.get_node_mut().parent.upgrade().unwrap();
                        drop(parent_borrow);
                        parent = pa;
                        continue;
                    }
                    _ => unreachable!(),
                };
                parent_widget.append(widget);
                return Ok(());
            }
        }
        VNodeType::TopLevelContainer(_) => {
            return Err("Can't add widget to top level container. Consider using a top level widget container. Eg. Body, Window");
        }
        // Widget has already been checked for in the init_member call
        VNodeType::Widget(_) => unreachable!(),
    };

    container_widget.append(widget);
    Ok(())
}
