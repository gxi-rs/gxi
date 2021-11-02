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
    fn get_children(&self) -> &Vec<StrongNodeType>;
    fn get_children_mut(&mut self) -> &mut Vec<StrongNodeType>;

    fn push(&mut self, member: StrongNodeType) {
        // do not add widget of to top level container widget
        match member.as_ref().borrow_mut().deref() {
            VNodeType::Widget(w) => {
                self.append_child(w.deref()).unwrap();
            }
            VNodeType::ContainerWidget(w) => {
                self.append_child(w.deref()).unwrap();
            }
            VNodeType::TopLevelContainerWidget(_) => (),
        }

        self.get_children_mut().push(member);
    }

    /// replaces or pushes member tp the given index. if the  index is equal to the amount
    /// of children, then the member is pushed. if no member is provided and push = false,
    /// then the member is removed.
    ///
    /// WARN: if push = false and member = None , then if tree doesn't contain an element at the
    /// location, then the next element might be removed
    fn set_at_index(&mut self, member: Option<StrongNodeType>, index: usize, push: bool) {
        let len = self.get_children().len();

        if index >= len {
            if let Some(member) = member {
                self.push(member)
            }
        } else if index < len {
            if let Some(member) = member {
                {
                    let children = self.get_children();
                    let old_member = children[index].as_ref().borrow();
                    let member_borrow = member.as_ref().borrow();

                    if push {
                        self.deref()
                            .insert_before(
                                member_borrow.get_native_widget(),
                                Some(old_member.get_native_widget()),
                            )
                            .unwrap();
                    } else {
                        self.replace_child(
                            member_borrow.get_native_widget(),
                            old_member.get_native_widget(),
                        )
                        .unwrap();
                    }
                }
                if push {
                    self.get_children_mut().insert(index, member);
                } else {
                    self.get_children_mut()[index] = member;
                }
            } else {
                if !push {
                    self.remove_child(self.get_children()[index].borrow().get_native_widget())
                        .unwrap();
                    self.get_children_mut().remove(index);
                }
            }
        } else {
            unreachable!()
        }
    }
}
