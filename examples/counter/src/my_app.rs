use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use rust_gui::{*, AsyncNode, c, gtk::prelude::*, NodeType};

pub struct MyApp {
    count: u32,
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub widget: gtk::Container,
}

impl Node for MyApp {
    impl_node_component!();

    fn new(parent_widget: Option<gtk::Container>) -> AsyncNode {
        Rc::new(RefCell::new(Box::new(Self {
            count: 10,
            child: None,
            sibling: None,
            widget: parent_widget.unwrap(),
        })))
    }

    fn render(top_state: AsyncNode) {
        let cont = Rc::clone(&top_state);
        let node = cont.clone();
        c!( View [
                View [
                    Button { set_label = "click"; connect_clicked = || state.count -= 1; }
                ],
                for x in 0..state.count
                    if x % 2 == 0
                        Button {set_label=&x.to_string();}
            ]
        );
    }
}
impl_drop_for_component!(MyApp);