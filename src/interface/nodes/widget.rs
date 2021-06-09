use crate::Node;
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::any::Any;

pub type GxiWidgetRc = Rc<RefCell<Box<dyn WidgetNode>>>;
pub type WeakGxiWidgetRc = Weak<RefCell<Box<dyn WidgetNode>>>;

/// Node which has a native widget
pub trait WidgetNode: Node {
    /// returns self or parents widget
    fn get_native_widget(&self) -> &dyn NativeWidget;
    fn get_native_widget_mut(&mut self) -> &mut dyn NativeWidget;
    fn as_widget_node(&self) -> &dyn WidgetNode;
    fn as_widget_node_mut(&mut self) -> &mut dyn WidgetNode;
}

/// Implemented on to native widgets like meta which can't have a child
pub trait NativeWidget {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
