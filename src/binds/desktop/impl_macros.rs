#[macro_export]
macro_rules! impl_drop {
    ($name:ident) => {
        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                     self.widget.0.destroy();
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_widget_node {
    ($name:ident) => {
        impl WidgetNode for $name {
            fn get_native_widget(&self) -> Box<dyn NativeWidget> {
                Box::from(self.widget.clone())
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
