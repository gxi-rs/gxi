use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use rust_gui::{c, gtk::prelude::*, AsyncNode, NodeType, *};

pub struct MyApp {
    count: u32,
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub parent: AsyncNode,
    pub widget: gtk::Container,
}

impl Node for MyApp {
    impl_node_component!();

    fn new(parent: AsyncNode, widget: Option<gtk::Container>) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(Self {
            count: 5,
            child: None,
            sibling: None,
            parent,
            widget: widget.unwrap(),
        })))
    }
    fn render(top_state: AsyncNode) {
        let cont = Rc::clone(&top_state);
        let node = cont.clone();
        c!( View [
            Button { set_label = "click"; connect_clicked = || state.count += 1; },
            {
                if state.count % 2 == 0 {
                    c! ( 1 Button { set_label="Eve"; } );
                } else {
                    c! ( 2 View );
                }
            }
        ]);
    }
}
/*
{
                forr! ( x in 0..state.count {
                    n! (Button init_sibling { set_label= &x.to_string();});
                });
                forr! ( x in 0..2 {
                    n! (View init_sibling {});
                });
}*/