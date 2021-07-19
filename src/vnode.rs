use std::any::Any;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};

use crate::{ContainerNode, NativeContainer, NativeWidget, VNodeType, WidgetNode};

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

/// VNode which hold's a native widget but can't be added to other nodes.
/// It itself can hold a node.
pub trait VTopLevelContainerWidget: VContainerWidget {}

/// VNode defined by the user
pub trait VComponent: VNode + AsRef<dyn VNode> + AsMut<dyn VNode> {
    fn get_node_ref(&self) -> &RefCell<ContainerNode>;
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
    fn get_node(&mut self) -> &ContainerNode;
    fn get_node_mut(&mut self) -> &ContainerNode;
}
