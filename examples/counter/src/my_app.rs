use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use rust_gui::{*, AsyncNode, c, gtk::prelude::*, NodeType};

use crate::odd_eve::OddEve;

pub struct MyApp {
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub parent: AsyncNode,
    pub widget: gtk::Container,
}

impl Node for MyApp {
    impl_node_component!();

    fn new(parent: AsyncNode, widget: Option<gtk::Container>) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(Self {
            child: None,
            sibling: None,
            parent,
            widget: widget.unwrap(),
        })))
    }

    fn render(top_state: AsyncNode) {
        let cont = Rc::clone(&top_state);
        let node = cont.clone();
        c!(
            View [
                OddEve
            ]
        );
    }
}
