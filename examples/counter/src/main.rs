use std::cell::RefCell;
use std::rc::Rc;

use rust_gui::{*, AsyncNode, c, Component, gtk::{prelude::*}};

fn main() {
    run::<MyAppState>();
}

#[derive(Default)]
struct MyAppState {
    count: i32,
}

impl Component for MyAppState {
    fn render(container: AsyncNode, top_state: Rc<RefCell<MyAppState>>) {
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
                Button { set_label = state.count.to_string().as_str(); connect_clicked = || state.count += 1; }
            ]
        ]
    );
    }
}

