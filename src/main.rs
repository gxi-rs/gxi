use std::cell::RefCell;
use std::rc::Rc;

use c::c;
use gtk::{ButtonExt, WidgetExt, WindowType};
use n::n;

use crate::nodes::{*};

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
    count: i32,
}

/*fn render(container: AsyncNode, state: Rc<RefCell<MyAppState>>) {
    let cont = Rc::clone(&container);
    let node = cont.clone();
    n!(View init_child { set_property_width_request = 300 ; });
    {
        let cont = node.clone();
        n!(Button init_child { set_label = state.count.to_string().as_str(); connect_clicked = || state.count += 1; });
        n!(Button init_sibling { set_label = state.count.to_string().as_str(); connect_clicked = || state.count += 1; });
        n!(View init_sibling {});
        {
            let cont = node.clone();
            n!(Button init_child { set_label = state.count.to_string().as_str(); connect_clicked = || state.count += 1; });
        }
    }
}*/

fn render(container: AsyncNode, state: Rc<RefCell<MyAppState>>) {
    let cont = Rc::clone(&container);
    let node = cont.clone();
    /*fn render(container: AsyncNode, state: Rc<RefCell<MyAppState>>) {
        let cont = Rc::clone(&container);
        let node = cont.clone();
        let node = {
            let (node, is_new) = {
                let mut node_borrow = node.as_ref().borrow_mut();
                let cont = Rc::clone(&cont);
                node_borrow.init_child(Box::new(move || View::new(cont.clone())), true)
            };
            {
                let mut node_borrow = node.as_ref().borrow_mut();
                let node = node_borrow.as_any_mut().downcast_mut::<View>().unwrap();
                if is_new {}
                let state = state.as_ref().borrow();
            }
            node
        };
        {
            let node = {
                let (node, is_new) = {
                    let mut node_borrow = node.as_ref().borrow_mut();
                    let cont = Rc::clone(&cont);
                    node_borrow.init_child(Box::new(move || View::new(cont.clone())), true)
                };
                {
                    let mut node_borrow = node.as_ref().borrow_mut();
                    let node = node_borrow.as_any_mut().downcast_mut::<View>().unwrap();
                    if is_new {}
                    let state = state.as_ref().borrow();
                }
                node
            };
            {
                let node = {
                    let (node, is_new) = {
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let cont = Rc::clone(&cont);
                        node_borrow.init_child(Box::new(move || Button::new(cont.clone())), true)
                    };
                    {
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let node = node_borrow.as_any_mut().downcast_mut::<Button>().unwrap();
                        if is_new {
                            {
                                let container_clone = Rc::clone(&container);
                                let state_clone = Rc::clone(&state);
                                node.widget.connect_clicked(move |_| {
                                    let state = state_clone.clone();
                                    {
                                        let mut state = state.as_ref().borrow_mut();
                                        state.count += 1
                                    }
                                    render(container_clone.clone(), state.clone());
                                });
                            }
                        }
                        let state = state.as_ref().borrow();
                        node.widget.set_label(state.count.to_string().as_str());
                    }
                    node
                };
                let node = {
                    let (node, is_new) = {
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let cont = Rc::clone(&cont);
                        node_borrow.init_sibling(Box::new(move || View::new(cont.clone())), true)
                    };
                    {
                        let mut node_borrow = node.as_ref().borrow_mut();
                        let node = node_borrow.as_any_mut().downcast_mut::<View>().unwrap();
                        if is_new {}
                        let state = state.as_ref().borrow();
                    }
                    node
                };
            }
        };
    }*/

    c! {
        View [
            View [
                Button { set_label = state.count.to_string().as_str(); connect_clicked = || state.count += 1; },
                View [
                    Button { set_label = "Hello"; connect_clicked = || state.count += 1; },
                    Button { set_label = "World"; connect_clicked = || state.count += 1; },
                    View [
                        Button { set_label = "1"; connect_clicked = || state.count += 1; },
                        Button { set_label = "2"; connect_clicked = || state.count += 2; },
                    ],
                    View [
                        Button { set_label = "3"; connect_clicked = || state.count += 3; },
                        Button { set_label = "4"; connect_clicked = || state.count += 4; },
                    ]
                ]
            ]
        ]
    };
}


/*
fn render(top_container: AsyncNode, state: Rc<RefCell<MyAppState>>) {
    {
        let view = nod!(View(cont, init_child, state){}{});
        {
            let bt = nod!(Button(view, init_child, state){}{});
            let bt = nod!(Button(bt, init_sibling, state){}{set_label = count});
        }
        let bt = nod!(Button(view, init_sibling, state){}{});
    }
}*/

/*fn render(top_container: AsyncNode, state: Rc<RefCell<MyAppState>>) {
    let container = Rc::clone(&top_container);
    let container = {
        let mut container_borrow = container.as_ref().borrow_mut();
        let container = Rc::clone(&container);
        container_borrow
            .init_child(Box::new(move || View::new(container.clone())), true)
            .0
    };
    {
        let node = {
            let container = {
                let mut container_borrow = container.as_ref().borrow_mut();
                let container = Rc::clone(&container);
                container_borrow
                    .init_child(Box::new(move || View::new(container.clone())), true)
                    .0
            };
            //init children
            {
                let node = {
                    let (node, is_new) = {
                        let mut node_borrow = container.as_ref().borrow_mut();
                        let container = Rc::clone(&container);
                        node_borrow
                            .init_child(Box::new(move || Button::new(container.clone())), true)
                    };
                    {
                        let mut button_borrow = node.as_ref().borrow_mut();
                        let button = button_borrow.as_any_mut().downcast_mut::<Button>().unwrap();

                        if is_new {
                            //init constants
                            let top_container_clone = Rc::clone(&top_container);
                            button.widget.connect_clicked(move |_| {
                                let state = state_clone.clone();
                                {
                                    let mut state = state.as_ref().borrow_mut();
                                    state.count += 1;
                                }
                                render(top_container_clone.clone(), state.clone());
                            });
                        }
                        //init non constants
                        button
                            .widget
                            .set_label(state.as_ref().borrow().count.to_string().as_str());
                    }
                    node
                };
                {}
                let _node = {
                    let mut node_borrow = node.as_ref().borrow_mut();
                    let container = Rc::clone(&container);
                    node_borrow
                        .init_sibling(Box::new(move || Button::new(container.clone())), true)
                        .0
                };
            }
            container
        };
        //init siblings
        let node = {
            let mut node_borrow = node.as_ref().borrow_mut();
            let container = Rc::clone(&container);
            node_borrow
                .init_sibling(Box::new(move || Button::new(container.clone())), true)
                .0
        };
        let _node = {
            let pure = {
                let mut node_borrow = node.as_ref().borrow_mut();
                let container = Rc::clone(&container);
                node_borrow
                    .init_sibling(Box::new(move || Pure::new(container.clone())), false)
                    .0
            };
            //get state
            let state = state.as_ref().borrow();
            //condition
            if state.count % 2 == 0 {
                //1st if block
                let _node = {
                    let mut pure_borrow = pure.as_ref().borrow_mut();
                    {
                        let pure: &mut Pure =
                            pure_borrow.as_any_mut().downcast_mut::<Pure>().unwrap();
                        //destroy previous AsyncNode if previous if block was not this
                        if pure.current_index != 1 {
                            pure.remove_child();
                            pure.current_index = 1;
                        }
                    }
                    let container = Rc::clone(&pure);
                    pure_borrow
                        .init_child(Box::new(move || Button::new(container.clone())), true)
                        .0
                };
            } else {
                //2nd if block
                let _node = {
                    let mut pure_borrow = pure.as_ref().borrow_mut();
                    {
                        let pure = pure_borrow.as_any_mut().downcast_mut::<Pure>().unwrap();
                        //destroy previous AsyncNode if previous if block was not this
                        if pure.current_index != 2 {
                            if pure.child.is_some() {
                                let child = pure.child.as_ref().unwrap();
                                pure.get_widget_as_container()
                                    .remove(&child.as_ref().borrow().get_widget())
                            }
                            pure.current_index = 2;
                        }
                    }
                };
            }
            //return
            pure
        };
    }
}
*//*
In macro form above function can be written as

component! {
    View {
        View {
            Button ( on_click=|_|{ state.count += 1 }, label={self.count} ),
            Button
        },
        Button,
        if state.count >= 1 { Button }
    }
}

 */
