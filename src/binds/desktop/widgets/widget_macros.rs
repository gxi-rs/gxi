#[macro_export]
macro_rules! create_widget {
    ($name:ident) => {
        create_widget!($name, $name);
    };
    ($name:ident,$widget_name:ident) => {
        use std::any::Any;
        use std::cell::RefCell;
        use std::rc::Rc;

        pub struct $name {
            parent: WeakNodeType,
            sibling: Option<StrongNodeType>,
            widget: gtk::$widget_name,
        }

        impl_drop!($name);
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
        impl Node for $name {
            impl_node_trait_as_any!();
            impl_node_trait_as_node!();
            impl_node_getters!();

            fn new(parent: WeakNodeType) -> StrongNodeType {
                Rc::new(RefCell::new(GxiNodeType::Widget(Box::new(Self {
                    parent,
                    widget: gtk::$widget_name::new($($args)*),
                    sibling: None,
                }))))
            }
        }
        impl_widget_node!($name);
    };
}
