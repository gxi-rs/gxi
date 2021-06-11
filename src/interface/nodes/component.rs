use crate::{Node, Container, WeakNodeType};

/// A component is node which doesn't have a native widget
pub trait ComponentNode: Node + Container {
    // parent substitute is the the parent in which outer children are added
    fn get_self_substitute(&self) -> &Option<WeakNodeType>;
    fn get_self_substitute_mut(&mut self) -> &mut Option<WeakNodeType>;
    fn is_dirty(&self) -> bool { false }
    fn mark_dirty(&mut self) {}
    fn mark_clean(&mut self) {}
}
