use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use crate::*;

pub struct Body {
    pub parent: WeakNodeType,
    pub self_substitute: Option<WeakNodeType>,
    pub child: Option<StrongNodeType>,
    pub sibling: Option<StrongNodeType>,
    pub widget: web_sys::HtmlElement,
}

impl Node for Body {
    fn new(parent: WeakNodeType) -> StrongNodeType {
        Rc::new(RefCell::new(GxiNodeType::Container(Box::new(Self {
            parent,
            self_substitute: None,
            child: None,
            sibling: None,
            widget: {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                document.body().unwrap()
            },
        }))))
    }

    impl_node_trait_as_any!();
    impl_node_trait_as_node!();
    impl_node_getters!();
}

impl_container_node!(Body);
impl_component_node!(Body);
impl_container!(Body);
impl_widget_node!(Body);

impl GlobalAttributes for Body {
    fn get_widget_as_element(&self) -> &web_sys::Element {
        &self.widget
    }
}
