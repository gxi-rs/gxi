#[macro_export]
macro_rules! create_web_widget {
    ($name:ident) => {
        use std::any::Any;
        use std::cell::RefCell;
        use std::rc::Rc;

        pub struct $name {
            pub parent: WeakNodeRc,
            pub sibling: Option<NodeRc>,
            pub widget: web_sys::Element,
        }

        impl_drop_for_web_node!($name);
    };
}


#[macro_export]
macro_rules! impl_web_widget {
    ($name:ident $element_name:literal) => {
        impl Node for $name {
            impl_node_as_any!();
            impl_node_trait_init_sibling!();
            impl_node_trait_get_widget!();
            impl_node_trait_get_sibling!();

            fn add(&mut self, _child:NodeRc) {
                panic!(
                    "Attempt to a add node into {name}. {name} can't have a child.",
                    name = stringify!($name)
                );
            }

            fn init_child(&mut self, _f: Box<dyn FnOnce() -> NodeRc>) -> (NodeRc, bool) {
                panic!(
                    "Attempt to a add node into {name}. {name} can't have a child.",
                    name = stringify!($name)
                );
            }

            fn get_widget_as_container(&self) -> NativeWidgetContainer {
                panic!("{} is not a container", stringify!($name));
            }

            fn get_self_substitute(&self) -> NodeRc {
                panic!("{} can't have a child", stringify!($name));
            }

            fn set_self_substitute(&mut self, _self_substitute: NodeRc) {
                panic!("{} can't have a child", stringify!($name));
            }

            fn new(parent: WeakNodeRc) -> NodeRc {
                Rc::new(RefCell::new(Box::new(Self {
                    parent,
                    sibling: None,
                    widget: {
                        let window = web_sys::window().unwrap();
                        let document = window.document().unwrap();
                        document.create_element($element_name).unwrap()
                    },
                })))
            }
        }
    };
}