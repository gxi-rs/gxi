#[macro_export]
macro_rules! generate_on_func {
    ($name:ident $event_name:literal) => {
        generate_on_func!($name $event_name Event);
    };
    ($name:ident $event_name:literal $event:ident) => {
        #[allow(non_snake_case)]
        #[allow(dead_code)]
        #[inline]
        pub fn $name<F: Fn(web_sys::$event) + 'static>(&self, f: F) {
            self.on($event_name, f);
        }
    };
}
