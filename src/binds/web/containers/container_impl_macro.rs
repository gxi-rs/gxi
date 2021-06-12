#[macro_export]
macro_rules! create_web_container {
    ($name:ident) => {
        use std::any::Any;
        use std::cell::RefCell;
        use std::rc::Rc;
        use std::ops::Deref;

        pub struct $name {
            pub parent: WeakNodeType,
            pub self_substitute: Option<WeakNodeType>,
            pub child: Option<StrongNodeType>,
            pub sibling: Option<StrongNodeType>,
            pub widget: web_sys::Element,
        }

    };
}

#[macro_export]
macro_rules! impl_web_container {
    ($name:ident $element_name:literal) => {
        impl Node for $name {
            impl_node_trait_as_any!();
            impl_node_trait_as_node!();
            impl_node_getters!();

            fn new(parent: WeakNodeType) -> StrongNodeType {
                Rc::new(RefCell::new(GxiNodeType::ContainerWidget(Box::new(Self {
                    parent,
                    self_substitute: None,
                    child: None,
                    sibling: None,
                    widget: {
                        let window = web_sys::window().unwrap();
                        let document = window.document().unwrap();
                        document.create_element($element_name).unwrap()
                    },
                }))))
            }
        }

        impl_container_node!($name);
        impl_component_node!($name);
        impl_container!($name);
        impl_widget_node!($name);
        impl_widget_node_deref!($name web_sys::Element);

        impl GlobalAttributes for $name {
            fn get_widget_as_element(&self) -> &web_sys::Element {
                &self.widget
            }
        }

        impl_drop_for_web_node!($name);
    };
}

#[macro_export]
macro_rules! impl_container_node {
    ($name:ident) => {
        impl ContainerWidgetNode for $name {
            fn get_native_container(&self) -> &NativeContainer {
                self.widget.as_ref()
            }

            fn append(&mut self, widget: &NativeWidget) {
                self.widget.append_child(widget).unwrap();
            }
        }
    };
}