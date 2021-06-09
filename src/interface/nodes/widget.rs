use crate::{Node, NativeWidget};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type GxiWidgetRc = Rc<RefCell<Box<dyn WidgetNode>>>;
pub type WeakGxiWidgetRc = Weak<RefCell<Box<dyn WidgetNode>>>;

/// Node which has a native widget
pub trait WidgetNode: Node {
    /// returns self or parents widget
    fn get_native_widget(&self) -> &NativeWidget;
    fn as_widget_node(&self) -> &dyn WidgetNode;
    fn as_widget_node_mut(&mut self) -> &mut dyn WidgetNode;
}
