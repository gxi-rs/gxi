use crate::{GxiNode, Container};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type GxiComponentRc = Rc<RefCell<Box<dyn ComponentNode>>>;
pub type WeakGxiComponentRc = Weak<RefCell<Box<dyn ComponentNode>>>;

/// A component is node which doesn't have a native widget
pub trait ComponentNode: GxiNode + Container {
    fn is_dirty(&self) -> bool {
        false
    }
    fn mark_dirty(&mut self) {}
    fn mark_clean(&mut self) {}
}
