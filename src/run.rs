use std::cell::RefCell;
use std::rc::Rc;

use gtk::{WindowType,WidgetExt};

use crate::{Component, Window};

pub fn run<T: Component>() {
    gtk::init().unwrap();
    let window = Window::new(WindowType::Toplevel);
    //render
    {
        let my_app_state = Rc::new(RefCell::new(T::default()));
        T::render(window.clone(), my_app_state);
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