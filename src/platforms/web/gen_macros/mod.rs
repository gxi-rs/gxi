#[macro_export]
macro_rules! generate_on_func {
    ($name:ident $event_name:literal) => {
        generate_on_func!($name $event_name Event);
    };
    ($name:ident $event_name:literal $event:ident) => {
        #[allow(non_snake_case)]
        pub fn $name<F: FnMut(web_sys::$event) + 'static>(&mut self, f: F) {
            self.on($event_name, f);
        }
    };
}

#[macro_export]
macro_rules! generate_attr {
    ($name:ident) => {
        #[allow(non_snake_case)]
        pub fn $name<T: AsRef<str>>(&self, value: T) {
            self.set_attribute(stringify!($name), value.as_ref())
                .unwrap();
        }
    };
}
