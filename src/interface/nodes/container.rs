use crate::{GxiNodeType, WidgetNode, GxiNode};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type GxiContainerRc = Rc<RefCell<Box<dyn ContainerNode>>>;
pub type WeakGxiContainerRc = Weak<RefCell<Box<dyn ContainerNode>>>;

/// Node which has a native widget
/// and can hold children
pub trait ContainerNode: GxiNode + WidgetNode + Container {
}

/// a node which can hold children
pub trait Container {
    fn get_child(&self) -> &Option<GxiNodeType>;
    fn get_child_mut(&mut self) -> &mut Option<GxiNodeType>;
}
