use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use rust_gui::{*, AsyncNode, c, gtk::prelude::*};

pub struct HelloWorld {
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub parent: AsyncNode,
    pub widget: gtk::Container,
}

impl Node for HelloWorld {
    impl_node_container!();

    fn new(parent: AsyncNode, widget: Option<gtk::Container>) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(Self {
            child: None,
            sibling: None,
            parent,
            widget: widget.unwrap(),
        })))
    }

    fn render(top_state: AsyncNode) {
        let container = {
            let borrow = top_state.as_ref().borrow();
            let state = borrow.as_any().downcast_ref::<Self>().unwrap();
            state.parent.clone()
        };
        let cont = Rc::clone(&container);
        let node = cont.clone();
        c!(
            Button { set_label = "Hello World"; }
        );
    }
}
