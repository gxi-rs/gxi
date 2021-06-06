use crate::Node;

pub trait ComponentNode: Node {
    /// render state management
    fn is_dirty(&self) -> bool { false }
    fn mark_dirty(&mut self) {}
    fn mark_clean(&mut self) {}
}