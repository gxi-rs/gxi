use std::borrow::BorrowMut;
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
    let mut window = render(Window::new(WindowType::Toplevel));
    let win: &mut Window = window.as_any_mut().downcast_mut().unwrap();
    let win: &mut gtk::Window = &mut win.widget;
    win.show_all();
    gtk::main();
}

fn render(mut container: AsyncNode) -> AsyncNode {
    container.init_child(Box::new(|| Button::new()));
    container
}