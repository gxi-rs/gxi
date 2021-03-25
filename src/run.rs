use gtk::{WidgetExt, WindowType};

use crate::{Component, Window};

pub fn run<App: Component>() {
    gtk::init().unwrap();
    let window = Window::new(WindowType::Toplevel);
    //render
    {
        let app_state = App::new(window.clone());
        App::render(window.clone(), app_state);
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