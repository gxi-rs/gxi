#[macro_export]
macro_rules! impl_container_node {
    ($name:ident) => {
        impl ContainerWidgetNode for $name {
            fn get_native_container(&self) -> &NativeContainer {
                self.widget.as_ref()
            }

            fn append(&mut self, widget: &NativeWidget) {
                self.widget.add(widget);
            }
        }
    };
}