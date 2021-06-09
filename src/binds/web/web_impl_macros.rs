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

#[macro_export]
macro_rules! generate_on_func {
    ($name:ident $event:literal) => {
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #[inline]
        fn $name<F: Fn() + 'static>(&self, f: F) {
            self.on($event, f);
        }
    };
}

#[macro_export]
macro_rules! generate_attr {
    ($name:ident) => {
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #[inline]
        fn $name(&self, value: &str) {
            self.get_widget_as_element()
                .set_attribute(stringify!($name), value)
                .unwrap();
        }
    };
}

#[macro_export]
macro_rules! generate_pub_attr {
    ($name:ident) => {
        generate_pub_attr!($name &str ; stringify!($name));
    };
    ($name:ident $typ:ty) => {
        generate_pub_attr!($name $typ ; stringify!($name));
    };
    ($name:ident $typ:ty ; $key:expr) => {
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #[inline]
        pub fn $name(&self, value: $typ) {
            self.widget
                .set_attribute($key, &format!("{}",value)[..])
                .unwrap();
        }
    };
}

#[macro_export]
macro_rules! generate_pub_bool_attr {
    ($name:ident) => {
        generate_pub_bool_attr!($name stringify!($name));
    };
    ($name:ident $key:expr) => {
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #[inline]
        pub fn $name(&self, value: bool) {
            if value {
                self.widget.set_attribute($key, "").unwrap();
            } else {
                self.widget.remove_attribute($key).unwrap();
            }
        }
    };
}

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
