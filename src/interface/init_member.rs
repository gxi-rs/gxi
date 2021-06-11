use std::borrow::{Borrow};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use crate::{GxiNodeType, StrongNodeType, WeakNodeType, InitType};

// TODO: replace init_type with f32 index
/// if init_type doesn't already exist then run init() and return clone of the new member
///
/// @return
/// + bool: false if child already exists
pub fn init_member<F: FnOnce(WeakNodeType) -> StrongNodeType>(
    this: StrongNodeType, init_type: InitType, init: F,
) -> (StrongNodeType, bool) {
    let mut this_borrow_mut = this.as_ref().borrow_mut();
    // check if this is a widget
    if let GxiNodeType::Widget(_) = this_borrow_mut.deref() {
        panic!("can't add node to a widget");
    }
    // add accordingly
    match init_type {
        InitType::Child => {
            // check if child already exists
            if let Some(child) = this_borrow_mut.as_container().unwrap().get_child() {
                return (child.clone(), false);
            }
            // if child does not exist initialize it
            let child = init(Rc::downgrade(&this));
            // if child is a widget or a container add it's widget to this if this is also a widget
            if let Ok(child) = child.as_ref().borrow().as_widget_node() {
                let child_borrow = child.borrow();

                match this_borrow_mut.deref_mut() {
                    GxiNodeType::Container(this) => {
                        this.append(child_borrow.get_native_widget());
                        drop(this_borrow_mut);
                    }
                    GxiNodeType::Component(this_deref) => {
                        drop(this_deref);
                        drop(this_borrow_mut);
                        let mut this = this.clone();
                        // get parent and init_member on it
                        loop {
                            println!("component");
                            let mut this_borrow_mut = this.as_ref().borrow_mut();
                            let parent = this_borrow_mut.as_node_mut().get_parent().upgrade().unwrap();
                            let mut parent_borrow = parent.as_ref().borrow_mut();
                            if let GxiNodeType::Container(parent) = parent_borrow.deref_mut() {
                                parent.append(child_borrow.get_native_widget());
                                break;
                            }
                            drop(parent_borrow);
                            drop(this_borrow_mut);
                            this = parent;
                        }
                    }
                    // this has already been checked
                    _ => unreachable!(),
                }
            } else {
                drop(this_borrow_mut);
            }
            println!("adding to container");
            let mut this_borrow_mut = this.as_ref().borrow_mut();
            *this_borrow_mut.as_container_mut().unwrap().get_child_mut() = Some(child.clone());
            return (child, true);
        }
        _ => {
            todo!()
        }
    }
}
