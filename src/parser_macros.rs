///! macros used with gxi_macro

/// initialises the component
#[doc(hidden)]
#[macro_export]
macro_rules! comp_new {
    ($state_name:ident $state_cell:ident $state_cell_inner:ident $parent:ident { $($sender:tt)* } { $($p:ident : $t:ty = $v:expr)* }) => {
        Rc::new(RefCell::new(Box::new(Self {
            state: $state_cell::new($state_cell_inner::new($state_name {
                $($p:$v),*
            })),
            $($sender)*
            self_substitute : None,
            $parent,
            dirty: true,
            child: None,
            sibling: None,
        })));
    };
}

/// creates the State Struct
#[doc(hidden)]
#[macro_export]
macro_rules! comp_state {
    ($state_name:ident { $($p:ident : $t:ty = $v:expr)* }) => {
        pub struct $state_name {
            $(pub $p:$t),*
        }
    };
}

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
