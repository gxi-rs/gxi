use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use gtk::{ButtonExt, WidgetExt, WindowType};
use gtk::prelude::WidgetExtManual;

use crate::nodes::containers::view::View;
use crate::nodes::containers::window::Window;
use crate::nodes::node::AsyncNode;
use crate::nodes::widgets::button::Button;

mod nodes;

fn main() {
    gtk::init().unwrap();
    let window = Window::new(WindowType::Toplevel);
    //render
    {
        let my_app_state = Rc::new(RefCell::new(MyAppState::default()));
        render(window.clone(), my_app_state);
    }
    //show window
    {
        let mut window_borrow = window.as_ref().borrow_mut();
        let window = window_borrow.as_any_mut().downcast_mut::<Window>().unwrap();
        window.widget.show_all();
    }
    //start main loop
    gtk::main();
}

#[derive(Default)]
struct MyAppState {
    count: i32
}

fn render(top_container: AsyncNode, state: Rc<RefCell<MyAppState>>) {
    {
        let container = Rc::clone(&top_container);
        let container = {
            let mut container_borrow = container.as_ref().borrow_mut();
            let container = Rc::clone(&container);
            container_borrow.init_child(Box::new(move || View::new(container.clone())))
        };
        {
            let node = {
                let container = {
                    let mut container_borrow = container.as_ref().borrow_mut();
                    let container = Rc::clone(&container);
                    container_borrow.init_child(Box::new(move || View::new(container.clone())))
                };
                //init children
                {
                    let node = {
                        let mut node_borrow = container.as_ref().borrow_mut();
                        let container = Rc::clone(&container);
                        node_borrow.init_child(Box::new(move || Button::new(container.clone())))
                    };
                    {
                        let mut button_borrow = node.as_ref().borrow_mut();
                        let button = button_borrow.as_any_mut().downcast_mut::<Button>().unwrap();
                        if state.as_ref().borrow().count == 0 {
                            let state_clone = Rc::clone(&state);
                            let top_container_clone = Rc::clone(&top_container);
                            button.widget.connect_clicked(move |_| {
                                let state = state_clone.clone();
                                {
                                    let mut state_borrow = state.as_ref().borrow_mut();
                                    state_borrow.count += 1;
                                }
                                render(top_container_clone.clone(), state.clone());
                            });
                        }
                        button.widget.set_label(state.as_ref().borrow().count.to_string().as_str())
                    }
                    let _node = {
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let container = Rc::clone(&container);
                        node_borrow.init_sibling(Box::new(move || Button::new(container.clone())))
                    };
                }
                container
            };

            //init siblings
            let node = {
                let mut node_borrow = node.as_ref().borrow_mut();
                let container = Rc::clone(&container);
                node_borrow.init_sibling(Box::new(move || Button::new(container.clone())))
            };
            let _node = {
                let mut node_borrow = node.as_ref().borrow_mut();
                let container = Rc::clone(&container);
                node_borrow.init_sibling(Box::new(move || Button::new(container.clone())))
            };
        }
    }
}
