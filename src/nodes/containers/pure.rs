use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::ContainerExt;

use crate::nodes::node::*;

pub struct Pure {
    pub parent: WeakNodeRc,
    //parent widget
    pub widget: gtk::Container,
    pub dirty: bool,
    pub child: Option<NodeRc>,
    pub sibling: Option<NodeRc>,
    pub pure_index: u32, //Index of current if block where 0 is default i.e when no if block was rendered before
}

impl Node for Pure {
    impl_node_component!();
    fn new(parent: WeakNodeRc) -> NodeRc {
        Rc::new(RefCell::new(Box::new(Pure {
            widget: parent
                .upgrade()
                .unwrap()
                .as_ref()
                .borrow()
                .get_widget_as_container(),
            parent,
            dirty: true,
            pure_index: 0,
            child: None,
            sibling: None,
        })))
    }
}

impl_drop_for_component!(Pure);
