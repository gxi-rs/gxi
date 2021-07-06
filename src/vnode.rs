use std::any::Any;
use std::cell::RefCell;

use crate::{Node, VNodeType};

/// anything that can fuck
pub trait VNode: AsRef<dyn Any> + AsMut<dyn Any> + 'static {
    fn new() -> Self
        where
            Self: Sized;
    fn into_vnode_type(self) -> VNodeType
        where
            Self: Sized;
}

/// anything that can fuck
pub trait VComponent: VNode + AsRef<dyn VNode> + AsMut<dyn VNode> {
    fn get_node_ref(&self) -> &RefCell<Node>;
}

/// anything that can fuck
pub trait VWidget: VNode + AsRef<dyn VNode> + AsMut<dyn VNode> {
    fn get_node(&self) -> &Node;
    fn get_node_mut(&mut self) -> &mut Node;
}
