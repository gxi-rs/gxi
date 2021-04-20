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
        #[inline]
        fn $name<F: Fn() + 'static>(&self, f: F) {
            self.on($event, f);
        }
    };
}

#[macro_export]
macro_rules! generate_attr {
    ($name:ident) => {
        #[inline]
        fn $name(&self, value: &str) {
            self.get_widget()
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
        #[inline]
        pub fn $name(&self, value: $typ) {
            self.get_widget()
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
        #[inline]
        pub fn $name(&self, value: bool) {
            if value {
                self.get_widget().set_attribute($key, "").unwrap();
            } else {
                self.get_widget().remove_attribute($key).unwrap();
            }
        }
    };
}

#[macro_export]
macro_rules! impl_add_for_web_node {
    () => {
        #[inline]
        fn add(&mut self, child: NodeRc) {
            self.widget
                .append_child(&child.as_ref().borrow().get_widget())
                .unwrap();
        }
    };
}
