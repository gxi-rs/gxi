#[macro_export]
macro_rules! impl_drop {
    ($name:ident) => {
        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    self.widget.destroy();
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_widget_node {
    ($name:ident) => {
        impl WidgetNode for $name {
            fn get_native_widget(&self) -> &NativeWidget {
                self.widget.as_ref()
            }
            fn as_widget_node(&self) -> &dyn WidgetNode {
                self
            }
            fn as_widget_node_mut(&mut self) -> &mut dyn WidgetNode {
                self
            }
        }
    };
}
