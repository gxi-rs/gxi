#[macro_export]
macro_rules! create_widget {
    ($name:ident) => {
        create_widget!($name, $name);
    };
    ($name:ident,$widget_name:ident) => {
        use std::any::Any;
        use std::cell::RefCell;
        use std::rc::Rc;

        use gtk::prelude::*;

        use crate::*;
        pub struct $name {
            pub parent: WeakNodeRc<Self>,
            pub dirty: bool,
            pub self_substitute: Option<WeakNodeRc<Self>>,
            pub widget: gtk::$widget_name,
            pub sibling: Option<NodeRc<Self>>,
        }

        impl_drop_for_node!($name);
    };
}

#[macro_export]
macro_rules! impl_widget {
    ($name:ident) => {
        impl_widget!($name, $name,() );
    };
    ($name:ident,( $($args:tt)* )) => {
        impl_widget!($name, $name, $($args:tt)*);
    };
    ($name:ident,$widget_name:ident,( $($args:tt)* )) => {
        type NativeWidget = gtk::Widget;
        type NativeWidgetContainer = gtk::Container;
        impl_node_trait!();
        impl_node_trait_init_sibling!();
        impl_node_trait_get_widget!();
        impl_node_trait_get_sibling!();
        fn add(&mut self, _child:NodeRc<Self>) {
            panic!(
                "Attempt to a add node into {}. {} can't have a child.",
                stringify!($name),
                stringify!($name)
            );
        }
        fn init_child(&mut self, _f: Box<dyn FnOnce() -> NodeRc<Self>>) -> (NodeRc<Self>, bool) {
            panic!(
                "Attempt to a add node into {}. {} can't have a child.",
                stringify!($name),
                stringify!($name)
            );
        }

        fn get_widget_as_container(&self) -> Self::NativeWidgetContainer {
            panic!("{} is not a container", stringify!($name));
        }

        fn new(parent: WeakNodeRc<Self>) -> NodeRc<Self> {
            Rc::new(RefCell::new(Box::new(Self {
                parent,
                self_substitute: None,
                dirty: true,
                widget: gtk::$widget_name::new($($args)*),
                sibling: None,
            })))
        }

        fn get_self_substitute(&self) -> NodeRc<Self> {
            panic!("{} can't have a child", stringify!($name));
        }
        fn set_self_substitute(&mut self, _self_substitute: NodeRc<Self>) {
            panic!("{} can't have a child", stringify!($name));
        }
    };
}
