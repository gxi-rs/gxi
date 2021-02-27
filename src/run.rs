use std::process::exit;
use std::sync::{Arc, Mutex};

use gtk::{ContainerExt, WidgetExt, WindowType};
use lazy_static::lazy_static;

use crate::components::Component;
use crate::MyApp;

lazy_static! {
    pub static ref APP: Mutex<MyApp> = Mutex::new(MyApp::default());
}

pub fn run() {
    gtk::init().unwrap();
    let win = gtk::Window::new(WindowType::Toplevel);
    {
        let mut app = APP.lock().unwrap();
        app.render();
        win.add(app.get_child().as_ref().unwrap().get_widget().unwrap());
    }
    win.connect_destroy(|_| exit(0));
    win.show_all();
    gtk::main();
}