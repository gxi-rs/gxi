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

/// downcast WeakNodeRc
#[doc(hidden)]
#[macro_export]
macro_rules! unwrap_weak_node {
    ($ident:ident as $ty:ty) => {
        let __node = $ident.upgrade().unwrap();
        let __node = __node.as_ref().borrow();
        let $ident = __node.as_node().as_any().downcast_ref::<$ty>().unwrap();
    };
}

/// mut downcast WeakNodeRc
#[doc(hidden)]
#[macro_export]
macro_rules! unwrap_weak_node_mut {
    ($ident:ident as $ty:ty) => {
        let __node = $ident.upgrade().unwrap();
        let mut __node = __node.as_ref().borrow_mut();
        let $ident = __node
            .as_node_mut()
            .as_any_mut()
            .downcast_mut::<$ty>()
            .unwrap();
    };
}
