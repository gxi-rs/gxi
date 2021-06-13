use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use crate::{GxiNodeType, StrongNodeType, WeakNodeType, InitType};
use std::cell::{Ref};

// TODO: replace init_type with f32 index
/// if init_type doesn't already exist then run init() and return clone of the new member
///
/// @return
/// + bool: false if child already exists
pub fn init_member<F: FnOnce(WeakNodeType) -> StrongNodeType>(
    this: StrongNodeType, init_type: InitType, init: F,
) -> (StrongNodeType, bool) {
    let mut this_borrow_mut = this.as_ref().borrow_mut();
    // add accordingly
    return match init_type {
        InitType::Child => {
            // check if this is a widget
            if let GxiNodeType::Widget(_) = this_borrow_mut.deref() {
                panic!("can't add node to a widget");
            }
            
            if let Some(child) = this_borrow_mut.as_container().unwrap().get_child() {
                return (child.clone(), false);
            }
            // create new child because it doesn't already exist
            let child = init(Rc::downgrade(&this));
            // add child
            *this_borrow_mut.as_container_mut().unwrap().get_child_mut() = Some(child.clone());
            drop(this_borrow_mut);
            // append native widget
            append_native_widget(this.clone(), child.as_ref().borrow());
            // return child
            (child, true)
        }
        InitType::Sibling => {
            // check if sibling already exists
            if let Some(sibling) = this_borrow_mut.as_node().get_sibling() {
                return (sibling.clone(), false);
            }
            let parent = this_borrow_mut.as_node_mut().get_parent().upgrade().unwrap();
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
                                let parent = this_borrow_mut.as_node_mut().get_parent().upgrade().unwrap();
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
