use std::any::Any;
use std::ops::{Deref, DerefMut};

use crate::{ContainerNode, NativeContainer, NativeWidget, TopLevelNode, VNodeType, WeakNodeType, WidgetNode};

/// Smallest node which can be added to other nodes but
/// it itself may or may not have the ability to hold a child
pub trait VNode: AsRef<dyn Any> + AsMut<dyn Any> + 'static {
    /// create a new instance of the node
    /// > TopLevelNode doesn't require a parent yet it needs to implement this function
    /// > to mantain a common interface for the gxi compiler
    fn new(parent: WeakNodeType) -> Self
    where
        Self: Sized;
    fn into_vnode_type(self) -> VNodeType
    where
        Self: Sized;
}

/// Node with refence to native widget and contains other nodes, but doesn't have a reference to it's parent. 
pub trait VTopLevelContainerWidget: 
    VNode + AsRef<dyn VNode> + AsMut<dyn VNode> + Deref<Target = NativeContainer> + DerefMut {
    fn get_node(&self) -> &TopLevelNode;
    fn get_node_mut(&mut self) -> &mut TopLevelNode;
}

/// Node which can contain other nodes, but doesn't have a reference to it's parent. 
pub trait VTopLevelContainer: 
    VNode + AsRef<dyn VNode> + AsMut<dyn VNode> {
    fn get_node(&self) -> &TopLevelNode;
    fn get_node_mut(&mut self) -> &mut TopLevelNode;
}

/// VNode defined by the user
pub trait VComponent: VNode + AsRef<dyn VNode> + AsMut<dyn VNode> {
    fn get_node(&self) -> &ContainerNode;
    fn get_node_mut(&mut self) -> &mut ContainerNode;
}

/// VNode referring to a native widget. It itself can't hold other widgets
pub trait VWidget:
    VNode + AsRef<dyn VNode> + AsMut<dyn VNode> + Deref<Target = NativeWidget> + DerefMut
{
    fn get_node(&self) -> &WidgetNode;
    fn get_node_mut(&mut self) -> &mut WidgetNode;
}

/// VNode referring to a native widget which can hold other widgets
pub trait VContainerWidget:
    VNode + AsRef<dyn VNode> + AsMut<dyn VNode> + Deref<Target = NativeContainer> + DerefMut
{
    fn get_node(&self) -> &ContainerNode;
    fn get_node_mut(&mut self) -> &mut ContainerNode;
}
