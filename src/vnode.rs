use std::any::Any;
use std::ops::{Deref, DerefMut};

use crate::{NativeContainerWidget, NativeWidget, TreeLeafNode, TreeNode, VNodeType};

/// Smallest node which can be added to other nodes but
/// it itself may or may not have the ability to hold a child
pub trait VNode: AsRef<dyn Any> + AsMut<dyn Any> + 'static {
    /// create a new instance of the node
    /// > TopLevelNode doesn't require a parent yet it needs to implement this function
    /// > to maintain a common interface for the gxi compiler
    fn new() -> Self
    where
        Self: Sized;
    fn into_vnode_type(self) -> VNodeType
    where
        Self: Sized;
}

/// VNode referring to a native widget. It itself can't hold other widgets
pub trait VWidget:
    VNode + AsRef<dyn VNode> + AsMut<dyn VNode> + Deref<Target = NativeWidget> + DerefMut
{
    fn get_node(&self) -> &TreeLeafNode;
    fn get_node_mut(&mut self) -> &mut TreeLeafNode;
}

/// VNode referring to a native widget which can hold other widgets
pub trait VContainerWidget:
    VNode + AsRef<dyn VNode> + AsMut<dyn VNode> + Deref<Target = NativeContainerWidget> + DerefMut
{
    fn get_node(&self) -> &TreeNode;
    fn get_node_mut(&mut self) -> &mut TreeNode;
}
