use std::ops::{Deref, DerefMut};

use crate::{NativeContainer, NativeWidget, Node};

/// Smallest node which can be added to other nodes but
/// it itself may or may not have the ability to hold a child
pub trait VNode {
    /// create a new instance of the node
    /// > TopLevelNode doesn't require a parent yet it needs to implement this function
    /// > to maintain a common interface for the gxi compiler
    fn new() -> Self
    where
        Self: Sized;

    fn as_node(&self) -> Node;
}

/// VNode referring to a native widget. It itself can't hold other widgets
pub trait VLeaf: VNode + Deref<Target = NativeWidget> + DerefMut {}

/// VNode referring to a native widget which can hold other widgets
pub trait VContainer: VNode + Deref<Target = NativeContainer> + DerefMut {
    fn push(&self, member: &Node, native_widget: &NativeWidget) {
        // do not add widget of to top level container widget
        match member {
            Node::Leaf => {
                #[cfg(feature = "web")]
                self.append_child(native_widget).unwrap();
            }
            Node::Container => {
                #[cfg(feature = "web")]
                self.append_child(native_widget).unwrap();
            }
            Node::TopLevelContainer => (),
        }
    }

    fn set_at_index(&self, _new_member: &Node, _index: usize, _replace: bool) {
        todo!()
        //        if replace {
        //            self.replace_child(
        //                new_member.as_ref().borrow().get_native_widget(),
        //                old_member.as_ref().borrow().get_native_widget(),
        //            )
        //            .unwrap();
        //        }
        //        let old_member = self.get_children()[index].clone();
        //        self.get_children_mut()[index] = new_member.clone();
        //
        //        match (old_member, new_member) {
        //            (Some(old_member), Some(new_member)) => {
        //                #[cfg(feature = "web")]
        //                self.replace_child(
        //                    new_member.as_ref().borrow().get_native_widget(),
        //                    old_member.as_ref().borrow().get_native_widget(),
        //                )
        //                .unwrap();
        //            }
        //            (None, Some(new_member)) => {
        //                let new_member_widget = new_member.as_ref().borrow();
        //                let new_member_widget = new_member_widget.deref().get_native_widget();
        //
        //                let children = self.get_children();
        //
        //                for i in (index + 1)..children.len() {
        //                    if let Some(next_member) = &children[i] {
        //                        #[cfg(feature = "web")]
        //                        self.deref()
        //                            .insert_before(
        //                                new_member_widget,
        //                                Some(next_member.as_ref().borrow().get_native_widget()),
        //                            )
        //                            .unwrap();
        //                        return;
        //                    }
        //                }
        //
        //                #[cfg(feature = "web")]
        //                self.append_child(new_member_widget).unwrap();
        //            }
        //            (Some(old_member), None) => {
        //                #[cfg(feature = "web")]
        //                self.remove_child(old_member.as_ref().borrow().get_native_widget())
        //                    .unwrap();
        //            }
        //            (None, None) => {}
        //        }
    }
}
