use std::any::Any;
use std::ops::{Deref, DerefMut};

use crate::{NativeContainerExt, NativeContainerWidget, NativeWidget, StrongNodeType, VNodeType};

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
        // do not add widget of ttop level container widget
        match member.as_ref().borrow_mut().deref() {
            VNodeType::Widget(w) => self.deref_mut().append(w.deref()),
            VNodeType::ContainerWidget(w) => self.deref_mut().append(w.deref()),
            VNodeType::TopLevelContainerWidget(_) => {}
        }

        self.get_children_mut().push(member);
    }
}
