use std::any::Any;
use std::ops::{Deref, DerefMut};

use crate::{NativeContainerWidget, NativeWidget, StrongNodeType, VNodeType};

/// Smallest node which can be added to other nodes but
/// it itself may or may not have the ability to hold a child
pub trait VNode: AsRef<dyn Any> + AsMut<dyn Any> + 'static {
    /// create a new instance of the node
    /// > TopLevelNode doesn't require a parent yet it needs to implement this function
    /// > to maintain a common interface for the gxi compiler
    fn new() -> Self
    where
        Self: Sized;
    fn into_strong_node_type(self) -> StrongNodeType;
}

/// VNode referring to a native widget. It itself can't hold other widgets
pub trait VWidget:
    VNode + AsRef<dyn VNode> + AsMut<dyn VNode> + Deref<Target = NativeWidget> + DerefMut
{
    fn push(&mut self, _member: StrongNodeType) {
        panic!("can't add child to this widget")
    }
}

/// VNode referring to a native widget which can hold other widgets
pub trait VContainerWidget:
    VNode + AsRef<dyn VNode> + AsMut<dyn VNode> + Deref<Target = NativeContainerWidget> + DerefMut
{
    fn get_children(&self) -> &Vec<Option<StrongNodeType>>;
    fn get_children_mut(&mut self) -> &mut Vec<Option<StrongNodeType>>;

    fn push(&mut self, member: Option<StrongNodeType>) {
        // do not add widget of to top level container widget
        if let Some(member) = &member {
            match member.as_ref().borrow_mut().deref() {
                VNodeType::Widget(w) => {
                    self.append_child(w.deref()).unwrap();
                }
                VNodeType::ContainerWidget(w) => {
                    self.append_child(w.deref()).unwrap();
                }
                VNodeType::TopLevelContainerWidget(_) => (),
            }
        }

        self.get_children_mut().push(member);
    }

    fn set_at_index(&mut self, new_member: Option<StrongNodeType>, index: usize) {
        match (&self.get_children()[index], new_member) {
            (Some(old_member), Some(new_member)) => {
                self.replace_child(
                    old_member.as_ref().borrow().get_native_widget(),
                    new_member.as_ref().borrow().get_native_widget(),
                )
                .unwrap();
                self.get_children_mut()[index] = Some(new_member);
            }
            (None, Some(_member)) => {
                todo!()
                // find nearst member
                //self.deref()
                //    .insert_before(
                //        member_borrow.get_native_widget(),
                //        Some(old_member.get_native_widget()),
                //    )
                //    .unwrap();
            }
            (Some(old_member), None) => {
                self.remove_child(old_member.as_ref().borrow().get_native_widget())
                    .unwrap();
                self.get_children_mut()[index] = Some(old_member.clone());
            }
            (None, None) => {}
        }
    }
}
