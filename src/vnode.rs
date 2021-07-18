use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::any::Any;

use crate::{NativeWidget, Node, VNodeType};

/// Smallest node which can be added to other nodes but
/// it itself may or may not have the ability to hold a child
pub trait VNode: AsRef<dyn Any> + AsMut<dyn Any> + 'static {
    fn new() -> Self
    where
        Self: Sized;
    fn into_vnode_type(self) -> VNodeType
    where
        Self: Sized;
}

/// Node which hold's a native widget but can't be added to other nodes.
/// It itself can hold a node.
pub trait VTopLevelWidget: VWidget {}

/// Node defined by the user
pub trait VComponent: VNode + AsRef<dyn VNode> + AsMut<dyn VNode> {
    fn get_node_ref(&self) -> &RefCell<Node>;
}

/// native widget
pub trait VWidget:
    VNode + 
    AsRef<dyn VNode>+
    AsMut<dyn VNode>+ 
    Deref<Target = NativeWidget> +
    DerefMut
{
    fn get_node(&self) -> &Node;
    fn get_node_mut(&mut self) -> &mut Node;
}
