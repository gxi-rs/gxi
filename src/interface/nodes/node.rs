use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::{GxiNodeType, WeakGxiNodeType};

pub type GxiNodeRc = Rc<RefCell<Box<dyn GxiNode>>>;
pub type WeakGxiNodeRc = Weak<RefCell<Box<dyn GxiNode>>>;

/// Struct should also implement Drop to remove node widget from the tree
pub trait GxiNode {
    /// @parent: Weak reference to parent, which allows GxiNode to add widget to parent further down the tree
    /// nodes can also have a new associated function with multiple params
    fn new(parent: WeakGxiNodeType) -> GxiNodeType
        where
            Self: Sized;
    /// type conversion
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    /// this: can't move &self to update closure therefore take a GxiNodeRc of Self
    fn render(_this: GxiNodeRc)
        where
            Self: Sized,
    {}
    // getters
    fn get_parent(&self) -> &WeakGxiNodeType;
    fn get_sibling(&self) -> &Option<GxiNodeType>;
    fn get_sibling_mut(&mut self) -> &mut Option<GxiNodeType>;
}
