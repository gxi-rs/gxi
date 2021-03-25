use gtk::{WidgetExt};

use crate::{Window, Node, AsyncNode};

pub fn run<App: Node>() {
    gtk::init().unwrap();
    let window:AsyncNode = Window::fake_new();
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