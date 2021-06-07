use crate::{InitType, WeakNodeType, StrongNodeType, GxiNodeType};
use std::ops::Deref;
use std::rc::Rc;
use std::borrow::{BorrowMut, Borrow};

// TODO: replace init_type with f32 index
/// if init_type doesn't already exist then run init() and return clone of the new member
///
/// @return
/// + bool: false if child already exists
pub fn init_member<F: FnOnce(WeakNodeType) -> StrongNodeType>(
    this: StrongNodeType, init_type: InitType, init: F,
) -> (StrongNodeType, bool) {
    let this_borrow = this.as_ref().borrow();
    match init_type {
        InitType::Child => {
            // check if child already exists
            match this_borrow.deref() {
                GxiNodeType::Component(this) => if let Some(child) = this.get_child().deref() {
                    return (child.clone(), false);
                }
                GxiNodeType::Container(this) => if let Some(child) = this.get_child().deref() {
                    return (child.clone(), false);
                }
                _ => panic!("Can't add a node into a widget")
            }
            // if child does not exist initialize it
            let child = init(Rc::downgrade(&this));
            // if child is a widget or a container add it's widget to this if this is also a widget
            match child.as_ref().borrow().deref() {
                GxiNodeType::Widget(child) | GxiNodeType::Container(child) => {

                }
                _ => ()
            }
            if let Ok(child) = child.clone().into_gxi_widget_rc() {
                let child_borrow = child.borrow();
                match &mut this {
                    GxiNodeType::Widget(this) => {
                        let mut this_borrow = this.as_ref().borrow_mut();
                        this_borrow.get_widget_mut().append(child_borrow.get_widget());
                    }
                    GxiNodeType::Component(_this) => {
                        // while parent isn't widget
                        // TODO: implement this
                        /*loop {
                            let mut this_borrow = this.borrow_mut();
                            let parent = this_borrow.get_parent().clone().upgrade();
                            let parent = parent.into_gxi_node_rc();
                        }*/
                    }
                    _ => unreachable!()
                }
            }

            let mut this_node_borrow = this_node.as_ref().borrow_mut();
            *this_node_borrow.get_child_mut() = Some(child.clone());
            return (child, true);
        }
        _ => {
            todo!()
        }
    }
}
