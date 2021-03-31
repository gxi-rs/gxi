use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use rust_gui::{*, AsyncNode, comp_init, gtk::prelude::*, NodeType};

comp! {
    MyApp {
        count : u32 = 0
    }
    render {
        View [
            View [
                Button ( set_label = "click", connect_clicked = || Msg::INC )
            ],
            for x in 0..state.count
                if x % 2 == 0
                    Button ( set_label=&x.to_string() )
        ]
    }
    update {
        state.count+=1;
        ShouldRender::Yes
    }
}

enum Msg { INC }



/*pub struct MyApp {
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
        c!(

        );
    }
}


impl_drop_for_component!(MyApp);*/