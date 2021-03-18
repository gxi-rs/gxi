use std::borrow::BorrowMut;





use gtk::{WidgetExt, WindowType};

use crate::nodes::containers::grid::Grid;
use crate::nodes::containers::window::Window;
use crate::nodes::node::{AsyncNode, Node};


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
    container_borrow.init_child(Box::new(move || Grid::new(container.clone())));
}

/*fn render(mut container: AsyncNode) {
    let mut container_clone = container.clone();
    let container_clone = container_clone.borrow_mut().get_mut();
    let container = container_clone.init_child(Box::new(|| Grid::new(container.clone())));
    {
        let bt = container_clone.init_child(Box::new(move || Button::new(Rc::clone(&container))));
        let bt = bt.as_ref().clone().borrow_mut().as_any_mut().downcast_mut::<Button>().unwrap();
        bt.widget.set_label("1");
    }
    {
        let bt = container_clone.init_child(Box::new(move || Button::new(Rc::clone(&container))));
        let bt = bt.as_ref().clone().borrow_mut().as_any_mut().downcast_mut::<Button>().unwrap();
        bt.widget.set_label("2");
    }
}*/
