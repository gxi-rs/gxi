use crate::{WidgetNode, Node, NativeWidget, Container};

/// Node which has a native widget
/// and can hold children
pub trait TopLevelWidgetNode: Node + WidgetNode + Container {
    fn append(&mut self, widget: &NativeWidget);
}
