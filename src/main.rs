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
    render(window.clone());
    let mut window = window.as_ref().clone().borrow_mut();
    let win: &mut Window = window.as_any_mut().downcast_mut().unwrap();
    let win: &mut gtk::Window = &mut win.widget;
    win.show_all();
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
