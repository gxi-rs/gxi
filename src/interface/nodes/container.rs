use crate::{WidgetNode, Node, StrongNodeType};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type GxiContainerRc = Rc<RefCell<Box<dyn ContainerNode>>>;
pub type WeakGxiContainerRc = Weak<RefCell<Box<dyn ContainerNode>>>;

/// Node which has a native widget
/// and can hold children
pub trait ContainerNode: Node + WidgetNode + Container {
}

/// a node which can hold children
pub trait Container: Node {
    fn get_child(&self) -> &Option<StrongNodeType>;
    fn get_child_mut(&mut self) -> &mut Option<StrongNodeType>;
}
