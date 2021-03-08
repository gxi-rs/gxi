use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

use gtk::{ContainerExt, WidgetExt, WindowType};

use crate::nodes::containers::grid::Grid;
use crate::nodes::containers::window::Window;
use crate::nodes::node::{AsyncNode, Node};
use crate::nodes::widgets::button::Button;
use std::borrow::BorrowMut;

mod nodes;

fn main() {
    gtk::init().unwrap();
    let window = Window::new(WindowType::Toplevel);

    render(window.clone());

    let mut window = window.clone();
    //window.as_ref().get_mut().as_any_mut().downcast_mut::<Window>().unwrap().widget.show_all();
    gtk::main();
}

fn render(container: AsyncNode) {
    let node = container.as_ref().clone();
    node.borrow_mut().init_child(Box::new(|| Button::new()));
   // container.get_widget_as_container().add(node.lock().unwrap().get_widget());
}