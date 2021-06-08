use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::{StrongNodeType, WeakNodeType};

pub type GxiNodeRc = Rc<RefCell<Box<dyn Node>>>;
pub type WeakGxiNodeRc = Weak<RefCell<Box<dyn Node>>>;

/// Struct should also implement Drop to remove node widget from the tree
pub trait Node {
    /// @parent: Weak reference to parent, which allows GxiNode to add widget to parent further down the tree
    /// nodes can also have a new associated function with multiple params
    fn new(parent: WeakNodeType) -> StrongNodeType
        where
            Self: Sized;
    /// type conversion
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    /// this: can't move &self to update closure therefore take a GxiNodeRc of Self
    fn render(_this: StrongNodeType)
        where
            Self: Sized,
    {}
    // getters
    fn get_parent(&self) -> &WeakNodeType;
    fn get_sibling(&self) -> &Option<StrongNodeType>;
    fn get_sibling_mut(&mut self) -> &mut Option<StrongNodeType>;
    // cast
    fn as_node(&self) -> &dyn Node;
    fn as_node_mut(&mut self) -> &mut dyn Node;
}
