use crate::{WidgetNode, Node, NativeContainer, NativeWidget, Container};

/// Node which has a native widget
/// and can hold children
pub trait ContainerWidgetNode: Node + WidgetNode + Container {
    fn get_native_container(&self) -> &NativeContainer;
    fn append(&mut self, widget: &NativeWidget);
}
