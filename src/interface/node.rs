use crate::{NodeType, WeakGxiNodeType, WidgetNode};
use std::any::Any;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type GxiNodeRc = Rc<RefCell<Box<dyn Node>>>;
pub type WeakGxiNodeRc = Weak<RefCell<Box<dyn Node>>>;
pub type GxiWidgetRc = Rc<RefCell<Box<dyn WidgetNode>>>;
pub type WeakGxiWidgetRc = Weak<RefCell<Box<dyn WidgetNode>>>;

/// Each struct which implements GxiNode should also have a new associated function
///
/// @parent: Weak reference to parent, which allows GxiNode to add widget to parent further down the tree
///
/// fn new(parent: WeakGxiNodeType, construct_values: Self::ConstructorParams) -> GxiNodeType where Self: Sized;
///
/// Struct should also implement Drop to remove node widget from the tree
pub trait Node {
    /// type conversion
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    /// this: can't move &self to update closure therefore take a GxiNodeRc of Self
    fn render(_this: GxiNodeRc)
    where
        Self: Sized,
    {
    }
    /// getters
    fn get_child(&self) -> &Option<NodeType>;
    fn get_child_mut(&mut self) -> &mut Option<NodeType>;
    fn get_parent(&self) -> WeakGxiNodeType;
}
