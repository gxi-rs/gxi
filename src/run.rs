use std::process::exit;

use gtk::{ContainerExt, WidgetExt, WindowType};

use crate::components::Component;

pub fn run<T: Component + Default>() {
    gtk::init().unwrap();
    let mut app = T::default();
    app.render();
    let win = gtk::Window::new(WindowType::Toplevel);
    win.add(app.get_child().as_ref().unwrap().get_widget().unwrap());
    win.connect_destroy(|_| exit(0));
    win.show_all();
    gtk::main();
}