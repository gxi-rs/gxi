use std::rc::Rc;

use gtk::{WidgetExt, WindowType};

use crate::nodes::containers::grid::View;
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
    let mut container_borrow = container_clone.as_ref().borrow_mut();
    {
        let container =
            container_borrow.init_child(Box::new(move || View::new(Rc::clone(&container))));
        let container_clone = container.clone();
        let mut container_borrow = container_clone.as_ref().borrow_mut();
        {
            let node = {
                let container = Rc::clone(&container);
                container_borrow.init_child(Box::new(move || Button::new(container.clone())))
            };
            drop(container_borrow);
            let node_clone = node.clone();
            let mut node_borrow = node_clone.as_ref().borrow_mut();
            {
                //init children if any here
            }
            //init siblings
            let node = {
                let node = {
                    let container = Rc::clone(&container);
                    node_borrow.init_sibling(Box::new(move || Button::new(container.clone())))
                };
                node
            };
            {
                let node_clone = node.clone();
                let mut node_borrow = node_clone.as_ref().borrow_mut();
                let _node = {
                    let container = Rc::clone(&container);
                    node_borrow.init_sibling(Box::new(move || Button::new(container.clone())))
                };
            }
        }
    }
}
