#[macro_export]
macro_rules! create_widget {
    ($name:ident) => {
        use std::any::Any;
        use std::cell::RefCell;
        use std::rc::Rc;
        use rust_gui_interface::*;

        pub struct $name {
            pub parent: WeakNodeRc,
            pub dirty: bool,
            pub self_substitute: Option<WeakNodeRc>,
            pub child: Option<NodeRc>,
            pub sibling: Option<NodeRc>,
            pub widget: web_sys::Element,
        }

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
macro_rules! impl_widget {
    ($name:ident $widget_block:block) => {
        impl Node for $name {
            impl_node_trait!();
            impl_node_trait_init_sibling!();
            impl_node_trait_init_child!();
            impl_node_trait_get_widget!();
            impl_node_trait_get_sibling!();
            impl_node_trait_get_child!();
            impl_node_trait_get_widget_as_container!();
            impl_node_trait_substitute!();

            fn new(parent: WeakNodeRc) -> NodeRc {
                Rc::new(RefCell::new(Box::new(Self {
                    parent,
                    dirty: false,
                    self_substitute: None,
                    child: None,
                    sibling: None,
                    widget: $widget_block,
                })))
            }

            fn add(&mut self, child: NodeRc) {
                self.widget
                    .append_child(&child.as_ref().borrow().get_widget())
                    .unwrap();
            }
        }
    };
}
