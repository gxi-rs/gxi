#[macro_export]
macro_rules! create_web_container {
    ($name:ident) => {
        use std::any::Any;
        use std::cell::RefCell;
        use std::rc::Rc;

        pub struct $name {
            pub parent: WeakNodeRc,
            pub self_substitute: Option<WeakNodeRc>,
            pub child: Option<NodeRc>,
            pub sibling: Option<NodeRc>,
            pub widget: web_sys::Element,
        }

        impl_drop_for_web_node!($name);
    };
}


#[macro_export]
macro_rules! impl_web_container {
    ($name:ident $element_name:literal) => {
        impl Node for $name {
            impl_node_as_any!();
            impl_node_trait_init_sibling!();
            impl_node_trait_init_child!();
            impl_node_trait_get_widget!();
            impl_node_trait_get_sibling!();
            impl_node_trait_get_child!();
            impl_node_trait_get_widget_as_container!();
            impl_node_trait_substitute!();

            fn add(&mut self, child: NodeRc) {
                self.widget.append_child(&child.as_ref().borrow().get_widget()).unwrap();
            }

            fn new(parent: WeakNodeRc) -> NodeRc {
                let this: NodeRc = Rc::new(RefCell::new(Box::new(Self {
                    parent,
                    self_substitute: None,
                    child: None,
                    sibling: None,
                    widget: {
                        let window = web_sys::window().unwrap();
                        let document = window.document().unwrap();
                        document.create_element(stringify!($name)).unwrap()
                    },
                })));
                {
                    let mut this_borrow = this.as_ref().borrow_mut();
                    this_borrow.set_self_substitute(this.clone());
                }
                this
            }
        }
    };
}
