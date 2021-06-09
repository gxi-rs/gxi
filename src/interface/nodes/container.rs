use crate::{WidgetNode, Node, StrongNodeType, NativeWidget};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type GxiContainerRc = Rc<RefCell<Box<dyn ContainerNode>>>;
pub type WeakGxiContainerRc = Weak<RefCell<Box<dyn ContainerNode>>>;

/// Node which has a native widget
/// and can hold children
pub trait ContainerNode: Node + WidgetNode + Container {
    fn get_native_container(&self) -> &dyn NativeContainer;
    fn get_native_container_mut(&mut self) -> &mut dyn NativeContainer;
}

/// a node which can hold children
pub trait Container: Node {
    fn get_child(&self) -> &Option<StrongNodeType>;
    fn get_child_mut(&mut self) -> &mut Option<StrongNodeType>;
    fn as_container(&self) -> &dyn Container;
    fn as_container_mut(&mut self) -> &mut dyn Container;
}

/// Implemented on to native widgets like Div which can have a child
pub trait NativeContainer: NativeWidget {
    fn append(&mut self, widget: &dyn NativeWidget);
}
