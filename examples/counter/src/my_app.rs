use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use rust_gui::{*, AsyncNode, c, gtk::prelude::*, NodeType};

use crate::hello_world::HelloWorld;

pub struct MyApp {
    count: i32,
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub parent: AsyncNode,
    pub widget: gtk::Container,
}

impl Node for MyApp {
    impl_node_component!();

    fn new(parent: AsyncNode, widget: Option<gtk::Container>) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(Self {
            count: 0,
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
                    Button { set_label = state.count.to_string().as_str(); connect_clicked = || state.count += 1; },
                    View [
                        HelloWorld,
                    ]
            ]
        );
    }
}
