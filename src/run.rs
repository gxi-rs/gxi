use std::process::exit;

use gtk::WidgetExt;

use crate::{NodeRc, Node, Window};

pub fn run<App: Node>() {
    gtk::init().unwrap();
    let window: NodeRc = Window::new(None);
    //render
    {
        let widget = Some(window.as_ref().borrow().get_widget_as_container());
        App::render(App::new(widget));
    }
    //show window
    {
        let mut window_borrow = window.as_ref().borrow_mut();
        let window = window_borrow.as_any_mut().downcast_mut::<Window>().unwrap();
        window.widget.connect_destroy(|_| {
            exit(0);
        });
        window.widget.show_all();
    }
    //start main loop
    gtk::main();
}
