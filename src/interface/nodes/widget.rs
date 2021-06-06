use std::any::Any;

use crate::{GxiNode};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type GxiWidgetRc = Rc<RefCell<Box<dyn WidgetNode>>>;
pub type WeakGxiWidgetRc = Weak<RefCell<Box<dyn WidgetNode>>>;

/// Node which has a native widget
pub trait WidgetNode: GxiNode {
    /// returns self or parents widget
    fn get_widget(&self) -> &dyn Widget;
    fn get_widget_mut(&mut self) -> &mut dyn Widget;
}

/// Implemented on to native widgets like H1, Body
pub trait Widget {
    fn append(&mut self, widget: &dyn Widget);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
