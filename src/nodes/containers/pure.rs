use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use gtk::{ContainerExt, WidgetExt};

use crate::nodes::node::{AsyncNode, Node, NodeType};

pub struct Pure {
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub parent: AsyncNode,
    pub widget: gtk::Container,
    pub current_index: u32, //Index of current if block where 0 is default i.e when no if block was rendered before
}

impl Node for Pure {
    impl_node_component!();
    fn new(parent: AsyncNode, widget: Option<gtk::Container>) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(Pure {
            current_index: 0,
            child: None,
            sibling: None,
            parent,
            widget: widget.unwrap(),
        })))
    }
}
