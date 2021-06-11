use crate::{Node, StrongNodeType};

/// a node which can hold children
pub trait Container: Node {
    fn get_child(&self) -> &Option<StrongNodeType>;
    fn get_child_mut(&mut self) -> &mut Option<StrongNodeType>;
    fn as_container(&self) -> &dyn Container;
    fn as_container_mut(&mut self) -> &mut dyn Container;
}
