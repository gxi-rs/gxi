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

        use crate::nodes::node::{AsyncNode, Node, NodeType};

        pub struct $name {
            pub dirty: bool,
            pub widget: gtk::$widget_name,
            pub sibling: Option<AsyncNode>,
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
        impl_node_trait!();
        impl_node_trait_init_sibling!();
        impl_node_trait_get_widget!();
        impl_node_trait_get_sibling!();

        fn init_child(
            &mut self, _f: Box<dyn FnOnce() -> AsyncNode>, _parent: gtk::Container,
        ) -> (AsyncNode, bool) {
            panic!(
                "Attempt to a add node into {}. {} can't have a child.",
                stringify!($name),
                stringify!($name)
            );
        }

        fn get_widget_as_container(&self) -> gtk::Container {
            panic!("{} is not a container", stringify!($name));
        }

        fn new(_parent_widget: Option<gtk::Container>) -> AsyncNode {
            Rc::new(RefCell::new(Box::new($name {
                dirty: true,
                widget: gtk::$widget_name::new($($args)*),
                sibling: None,
            })))
        }
    };
}
