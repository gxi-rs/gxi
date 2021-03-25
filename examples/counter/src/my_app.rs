use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use rust_gui::{*, AsyncNode, c, gtk::prelude::*};

use crate::hello_world::HelloWorld;

pub struct MyApp {
    count: i32,
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub parent: AsyncNode,
    pub widget: gtk::Container,
}

impl Node for MyApp {
    impl_node_container!();

    fn new(parent: AsyncNode, widget: Option<gtk::Container>) -> AsyncNode  {
        Rc::new(RefCell::new(Box::new(Self {
            count: 0,
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
            View [
                View [
                    {
                        if state.count % 2 == 0 {
                            c!(1 Button { set_label = "even"; connect_clicked = || state.count += 1; });
                        } else {
                            c!(2 View);
                        }
                    }
                    Button { set_label = state.count.to_string().as_str(); connect_clicked = || state.count += 1; },
                    HelloWorld,
                ]
            ]
        );
    }
}
