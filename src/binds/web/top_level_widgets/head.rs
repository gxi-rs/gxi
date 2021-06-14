use std::any::Any;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use crate::*;

pub struct Head {
    pub parent: WeakNodeType,
    pub self_substitute: Option<WeakNodeType>,
    pub child: Option<StrongNodeType>,
    pub sibling: Option<StrongNodeType>,
    pub widget: web_sys::HtmlHeadElement,
}

impl Node for Head {
    fn new(parent: WeakNodeType) -> StrongNodeType {
        Rc::new(RefCell::new(GxiNodeType::TopLevelWidget(Box::new(Self {
            parent,
            self_substitute: None,
            child: None,
            sibling: None,
            widget: {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                document.head().unwrap()
            },
        }))))
    }

    impl_node_trait_as_any!();
    impl_node_trait_as_node!();
    impl_node_getters!();
}

impl_container_node!(Head);
impl_component_node!(Head);
impl_container!(Head);
impl_widget_node!(Head);
impl_widget_node_deref!(Head web_sys::HtmlHeadElement);

impl GlobalAttributes for Head {
    fn get_widget_as_element(&self) -> &web_sys::Element {
        &self.widget
    }
}

impl Drop for Head {
    fn drop(&mut self) {
        //need not drop head tag
    }
}
