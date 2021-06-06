use std::any::Any;

use crate::Node;

/// Node which has a native widget
pub trait WidgetNode: Node {
    /// returns self or parents widget
    fn get_widget(&self) -> &dyn NativeWidget;
    fn get_widget_mut(&mut self) -> &mut dyn NativeWidget;
}

/// Implemented on to native widgets like H1, Body
pub trait NativeWidget {
    fn append(&mut self, widget: &dyn NativeWidget);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
