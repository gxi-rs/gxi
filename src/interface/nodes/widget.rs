use std::any::Any;

use crate::Node;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type GxiWidgetRc = Rc<RefCell<Box<dyn WidgetNode>>>;
pub type WeakGxiWidgetRc = Weak<RefCell<Box<dyn WidgetNode>>>;

/// Node which has a native widget
pub trait WidgetNode: Node {
    /// returns self or parents widget
    fn get_widget(&self) -> &dyn Widget;
    fn get_widget_mut(&mut self) -> &mut dyn Widget;
    fn as_widget_node(&self) -> &dyn WidgetNode;
    fn as_widget_node_mut(&mut self) -> &mut dyn WidgetNode;
}

/// Implemented on to native widgets like H1, Body
pub trait Widget {
    fn append(&mut self, widget: &dyn Widget);
}
