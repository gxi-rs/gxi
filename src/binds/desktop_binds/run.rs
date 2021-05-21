use std::cell::RefCell;
use std::rc::Rc;

use crate::{*};

pub fn run<App: Node + 'static>() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        gtk::init().unwrap();
        let fake_parent: NodeRc = Rc::new(RefCell::new(Box::new(Tree)));
        let window = Window::new(Rc::downgrade(&fake_parent));
        //render
        let this = App::new(Rc::downgrade(&window));
        App::render(this.clone());
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
