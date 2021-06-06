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
            sibling: Option<NodeType>,
            widget: GtkWidget<gtk::$widget_name>
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

            fn get_parent(&self) -> &WeakNodeType {
                &self.parent
            }

            fn new(parent: WeakNodeType) -> NodeType {
                NodeType::Widget(Rc::new(RefCell::new(Box::new(Self {
                    parent,
                    widget: GtkWidget(gtk::$widget_name::new($($args)*)),
                    sibling: None,
                }))))
            }
        }
        impl_widget_node!($name);
    };
}
