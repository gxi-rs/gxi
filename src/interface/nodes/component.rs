use crate::{GxiNode, Container, WeakGxiNodeType};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type GxiComponentRc = Rc<RefCell<Box<dyn ComponentNode>>>;
pub type WeakGxiComponentRc = Weak<RefCell<Box<dyn ComponentNode>>>;

/// A component is node which doesn't have a native widget
pub trait ComponentNode: GxiNode + Container {
    // parent substitute is the the parent in which outer children are added
    fn get_self_substitute(&self) -> &Option<WeakGxiNodeType>;
    fn get_self_substitute_mut(&mut self) -> &mut Option<WeakGxiNodeType>;

    fn is_dirty(&self) -> bool {
        false
    }
    fn mark_dirty(&mut self) {}
    fn mark_clean(&mut self) {}
}
