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
            fn get_widget(&self) -> &dyn Widget {
                &self.widget
            }
            fn get_widget_mut(&mut self) -> &mut dyn Widget {
                &mut self.widget
            }
        }
    };
}
