use crate::{GxiNodeType, InitType, StrongNodeType, WeakNodeType};
use std::borrow::{Borrow};
use std::ops::{Deref,DerefMut};
use std::rc::Rc;

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
        panic!("can't add node to widget");
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
                        this.get_widget_mut().append(child_borrow.get_widget());
                    }
                    GxiNodeType::Component(_this) => {
                        // while parent isn't widget
                        // TODO: implement this
                        /*loop {
                            let mut this_borrow = this.borrow_mut();
                            let parent = this_borrow.get_parent().clone().upgrade();
                            let parent = ent.into_gxi_node_rc();
                        }*/
                    }
                    _ => unreachable!(),
                }
            }
            println!("adding to container");
            *this_borrow_mut.as_container_mut().unwrap().get_child_mut() = Some(child.clone());
            return (child, true);
        }
        _ => {
            todo!()
        }
    }
}
