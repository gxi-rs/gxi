use crate::*;
use std::any::Any;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

pub struct Html {
    pub parent: WeakNodeType,
    pub self_substitute: Option<WeakNodeType>,
    pub child: Option<StrongNodeType>,
    pub sibling: Option<StrongNodeType>,
    pub widget: web_sys::Element,
}

impl Node for Html {
    fn new(parent: WeakNodeType) -> StrongNodeType {
        Rc::new(RefCell::new(GxiNodeType::TopLevelWidget(Box::new(Self {
            parent,
            self_substitute: None,
            child: None,
            sibling: None,
            widget: {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                document.get_elements_by_tag_name("html").item(0).unwrap()
            },
        }))))
    }

    impl_node_trait_as_any!();
    impl_node_trait_as_node!();
    impl_node_getters!();
}

impl_container_node!(Html);
impl_component_node!(Html);
impl_container!(Html);
impl_widget_node!(Html);
impl_widget_node_deref!(Html web_sys::Element);

impl Drop for Html {
    fn drop(&mut self) {
        //need not drop head tag
    }
}

impl GlobalAttributes for Html {
    fn get_widget_as_element(&self) -> &web_sys::Element {
        &self.widget
    }
}

impl Html {
    generate_pub_attr!(lang);
    generate_pub_attr!(xmlns);
}
