use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

use gtk::{ContainerExt, WidgetExt, WindowType};

use crate::nodes::containers::grid::Grid;
use crate::nodes::containers::window::Window;
use crate::nodes::node::{AsyncNode, Node};
use crate::nodes::widgets::button::Button;

mod nodes;

fn main() {
    gtk::init().unwrap();
    let window: AsyncNode = Arc::new(Mutex::new(Box::new(Window::new(
        WindowType::Toplevel,
    ))));

    render(window.clone());

    window.clone().lock().unwrap().as_any_mut().downcast_mut::<Window>().unwrap().widget.show_all();
    gtk::main();
}

fn render(container_super: AsyncNode) {
    let mut container = container_super.lock().unwrap();
    let node = container.get_child_mut().get_or_insert_with(|| Button::new(container_super.clone())).clone();
    container.get_widget_as_container().add(node.lock().unwrap().get_widget());
}