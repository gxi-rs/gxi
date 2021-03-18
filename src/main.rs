use std::rc::Rc;

use gtk::{WidgetExt, WindowType};

use crate::nodes::containers::view::View;
use crate::nodes::containers::window::Window;
use crate::nodes::node::AsyncNode;
use crate::nodes::widgets::button::Button;

mod nodes;

fn main() {
    gtk::init().unwrap();
    let window = Window::new(WindowType::Toplevel);
    //render
    render(window.clone());
    //show window
    {
        let mut window_borrow = window.as_ref().borrow_mut();
        let window = window_borrow.as_any_mut().downcast_mut::<Window>().unwrap();
        window.widget.show_all();
    }
    //start main loop
    gtk::main();
}

fn render(container: AsyncNode) {
    let container_clone = container.clone();
    {
        let container = {
            let mut container_borrow = container_clone.as_ref().borrow_mut();
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
