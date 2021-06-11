use crate::{WidgetNode, Node, StrongNodeType, binds::NativeContainer, NativeWidget};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type GxiContainerRc = Rc<RefCell<Box<dyn ContainerWidgetNode>>>;
pub type WeakGxiContainerRc = Weak<RefCell<Box<dyn ContainerWidgetNode>>>;

/// Node which has a native widget
/// and can hold children
pub trait ContainerWidgetNode: Node + WidgetNode + Container {
    fn get_native_container(&self) -> &NativeContainer;
    fn append(&mut self, widget: &NativeWidget);
}

/// a node which can hold children
pub trait Container: Node {
    fn get_child(&self) -> &Option<StrongNodeType>;
    fn get_child_mut(&mut self) -> &mut Option<StrongNodeType>;
    fn as_container(&self) -> &dyn Container;
    fn as_container_mut(&mut self) -> &mut dyn Container;
}
