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
                View [
                    Button { set_label = "click"; connect_clicked = || state.count += 1; }
                ],
                if state.count % 5 == 0
                    if state.count == 5
                        Button { set_label = "FIVE"; }
                else if state.count % 2 == 0
                    if state.count == 2
                        Button { set_label = "Two"; }
                    else
                        Button { set_label = "Any other mULTIPLE OF 2"; }
            ]
        );
    }
}
/*
{
    Node
    {
        Pure Block sibling of node, This is container
        --> Child of Pure Block is now node
        //iff!()
        {
            Pure Block sibling of node
            if Block
                child of node
            else
                child of node
        }
        //forr
        {
            Pure Block sibling of node now cont
            Pure Block child of cont now node
            for x in y
                sibling of node
        }
    }
}*/
