#[macro_export]
macro_rules! impl_drop_for_web_node {
    ($name:ident) => {
        impl Drop for $name {
            fn drop(&mut self) {
                self.widget
                    .parent_node()
                    .unwrap()
                    .remove_child(&self.widget)
                    .unwrap();
            }
        }
    };
}
