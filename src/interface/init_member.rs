use crate::{GxiNodeType, InitType, StrongNodeType, WeakNodeType};
use std::cell::Ref;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

// TODO: replace init_type with f32 index
/// if init_type doesn't already exist then run init() and return clone of the new member
///
/// @return
/// + bool: false if member already exists
pub fn init_member<F: FnOnce(WeakNodeType) -> StrongNodeType>(
    mut this: StrongNodeType,
    init_type: InitType,
    init: F,
    is_this_root: bool,
) -> (StrongNodeType, bool) {
    // add accordingly
    return match init_type {
        InitType::Child => {
            // check if this is a widget
            {
                let this_borrow = this.clone();
                let this_borrow = this_borrow.as_ref().borrow();
                match this_borrow.deref() {
                    GxiNodeType::Widget(_) => panic!("can't add node to a widget"),
                    // if this is a component and isn't root then get self_substitute
                    GxiNodeType::Component(comp) => {
                        if !is_this_root {
                            let subst = comp
                                .get_self_substitute()
                                .as_ref()
                                .expect("You are trying to add a child into component which doen't support #children");
                            this = subst.upgrade().expect(
                                "#children no longer exists. Make sure it lives long enough",
                            );
                        }
                    }
                    _ => (),
                }
            }

            let mut this_borrow_mut = this.as_ref().borrow_mut();
            // self_substitute already checked, so return child
            if let Some(child) = this_borrow_mut.as_container().unwrap().get_child() {
                return (child.clone(), false);
            }

            // create new child because it doesn't already exist
            let child = init(Rc::downgrade(&this));
            // if
            *this_borrow_mut.as_container_mut().unwrap().get_child_mut() = Some(child.clone());
            drop(this_borrow_mut);
            // append native widget
            append_native_widget(this.clone(), child.as_ref().borrow());
            // return child
            (child, true)
        }
        InitType::Sibling => {
            let mut this_borrow_mut = this.as_ref().borrow_mut();
            // check if sibling already exists
            if let Some(sibling) = this_borrow_mut.as_node().get_sibling() {
                return (sibling.clone(), false);
            }
            let parent = this_borrow_mut
                .as_node_mut()
                .get_parent()
                .upgrade()
                .unwrap();
            // create new sibling because it doesn't already exist
            let sibling = init(Rc::downgrade(&parent));
            // add sibling
            *this_borrow_mut.as_node_mut().get_sibling_mut() = Some(sibling.clone());
            drop(this_borrow_mut);
            // append native widget
            append_native_widget(parent, sibling.as_ref().borrow());
            // return sibling
            (sibling, true)
        }
    };
}

fn append_native_widget(mut this: StrongNodeType, member_borrow: Ref<GxiNodeType>) {
    let mut this_borrow_mut = this.as_ref().borrow_mut();
    if let GxiNodeType::TopLevelWidget(_) = member_borrow.deref() {
        // no need to add top level widget's native widget
    } else {
        // if child is a widget or a container add it's widget to this if this is also a widget
        if let Ok(member) = member_borrow.as_widget_node() {
            // if this is a container widget node then append child's native widget node to it
            if let Ok(this) = this_borrow_mut.as_container_widget_node_mut() {
                this.append(member.get_native_widget());
            } else {
                match this_borrow_mut.deref_mut() {
                    GxiNodeType::Component(_) => {
                        drop(this_borrow_mut);
                        loop {
                            this = {
                                let mut this_borrow_mut = this.as_ref().borrow_mut();
                                let parent = this_borrow_mut
                                    .as_node_mut()
                                    .get_parent()
                                    .upgrade()
                                    .unwrap();
                                let mut parent_borrow = parent.as_ref().borrow_mut();
                                if let Ok(parent) = parent_borrow.as_container_widget_node_mut() {
                                    parent.append(member.get_native_widget());
                                    break;
                                }
                                drop(this_borrow_mut);
                                drop(parent_borrow);
                                parent
                            };
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}
