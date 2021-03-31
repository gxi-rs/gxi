use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;

use rust_gui::{*, AsyncNode, comp_init, gtk::prelude::*, NodeType};

enum Msg { INC }

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