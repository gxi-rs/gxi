use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::{ContainerExt};

use crate::nodes::node::{*};

pub struct Pure {
    pub dirty: bool,
    pub child: Option<NodeRc>,
    pub sibling: Option<NodeRc>,
    pub widget: gtk::Container,
    pub pure_index: u32, //Index of current if block where 0 is default i.e when no if block was rendered before
}

impl Node for Pure {
    impl_node_component!();
    fn new(parent_widget: Option<gtk::Container>) -> NodeRc {
        Rc::new(RefCell::new(Box::new(Pure {
            dirty: true,
            pure_index: 0,
            child: None,
            sibling: None,
            widget: parent_widget.unwrap(),
        })))
    }
}

impl_drop_for_component!(Pure);
