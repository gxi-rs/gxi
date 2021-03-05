use std::ops::{DerefMut};
use std::sync::{Arc, Mutex};

use gtk::{WidgetExt, WindowType};

use crate::nodes::containers::grid::Grid;
use crate::nodes::containers::window::Window;
use crate::nodes::node::{AsyncNode, Node};
use crate::nodes::widgets::button::Button;

mod nodes;

fn main() {
    gtk::init().unwrap();
    let window: AsyncNode = Arc::new(Mutex::new(Node::Container(Box::new(Window::new(
        WindowType::Toplevel,
    )))));

    let container: AsyncNode = Arc::new(Mutex::new(Node::Container(Box::new(Grid::new(
        window.clone(),
    )))));

    if let Node::Container(s) = container.as_ref().clone().lock().unwrap().deref_mut() {
        s.get_child_mut().get_or_insert_with(|| {
            Arc::new(Mutex::new(Node::Widget(Box::new(Button::new(
                container.clone(),
            )))))
        });
    };

    if let Node::Container(window) = window.clone().lock().unwrap().deref_mut() {
        let window = window.as_any_mut().downcast_mut::<Window>().unwrap();
        window.widget.show_all();
        gtk::main();
    }
}
