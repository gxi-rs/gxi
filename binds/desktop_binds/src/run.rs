use std::cell::RefCell;
use std::rc::Rc;

use gtk::WidgetExt;
use rust_gui_interface::{gtk, Fake, NodeRc};

use crate::{Node, Window};

pub fn run<App: Node + 'static>() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        gtk::init().unwrap();
        let fake_parent: NodeRc = Rc::new(RefCell::new(Box::new(Fake)));
        let window = Window::new(Rc::downgrade(&fake_parent));
        //render
        {
            App::render(App::new(Rc::downgrade(&window)));
        }
        //show window
        {
            let mut window_borrow = window.as_ref().borrow_mut();
            let window = window_borrow.as_any_mut().downcast_mut::<Window>().unwrap();
            window.widget.connect_destroy(|_| {
                gtk::main_quit();
            });
            window.widget.show_all();
        }
        //start main loop
        gtk::main();
    });
}
