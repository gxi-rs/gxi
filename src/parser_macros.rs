///! macros used with gxi_macro

/// for Arc<Mutex<State>>>
#[doc(hidden)]
#[macro_export]
macro_rules! get_arc_state {
    ($($state:tt)*) => {
        $($state)*.lock().unwrap();
    };
}

/// for RC<RefCell<State>>>
#[doc(hidden)]
#[macro_export]
macro_rules! get_rc_state {
    ($($state:tt)*) => {
        $($state)*.as_ref().borrow();
    };
}
/// mut for RC<RefCell<State>>>
#[doc(hidden)]
#[macro_export]
macro_rules! get_mut_rc_state {
    ($($state:tt)*) => {
        $($state)*.as_ref().borrow_mut();
    };
}
