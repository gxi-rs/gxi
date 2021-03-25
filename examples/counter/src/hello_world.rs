use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use rust_gui::{*, AsyncNode, c, gtk::prelude::*};

pub struct HelloWorld {
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub parent: AsyncNode,
}

impl Node for HelloWorld {
    impl_node_container!();

    fn new(parent: AsyncNode) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(Self {
            child: None,
            sibling: None,
            parent,
        })))
    }

    fn render(top_state: AsyncNode) {
        let container = top_state.as_ref().borrow().as_any().downcast_ref::<Self>().unwrap().parent.clone();
        let cont = Rc::clone(&container);
        let node = cont.clone();
        c!(
            View [
                Button { set_label = "Hello World"; }
            ]
        );
    }
}