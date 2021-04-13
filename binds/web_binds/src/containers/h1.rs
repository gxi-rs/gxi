use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use rust_gui_interface::{*};

pub struct H1 {
    pub parent: WeakNodeRc,
    pub dirty: bool,
    pub self_substitute: Option<WeakNodeRc>,
    pub child: Option<NodeRc>,
    pub sibling: Option<NodeRc>,
    pub widget: web_sys::Element,
}

impl Node for H1 {
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
            widget: {
                let window = web_sys::window().unwrap();
                let document = window.document().unwrap();
                document.create_element("h1").unwrap()
            },
        })))
    }

    fn add(&mut self, child: NodeRc) {
        self.widget.append_child(&child.as_ref().borrow().get_widget()).unwrap();
    }
}

impl H1 {
    pub fn label(&self, text: &str) {
        self.widget.set_inner_html(&text);
    }
}

impl Drop for H1 {
    fn drop(&mut self) {
        self.widget.parent_node().unwrap().remove_child(&self.widget).unwrap();
    }
}