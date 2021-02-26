use gtk::{ContainerExt, WindowType, WidgetExt};

use crate::components::Component;
use crate::MyApp;

pub fn run<T: Component + Default>() {
    gtk::init();
    let mut app = T::default();
    app.render();
    let win = gtk::Window::new(WindowType::Toplevel);
    win.add(app.get_child().as_ref().unwrap().get_widget().unwrap());
    win.show_all();
    gtk::main();
}