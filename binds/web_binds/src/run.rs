use std::cell::RefCell;
use std::rc::Rc;

use rust_gui_interface::{Fake, Node, NodeRc, runtime};

pub fn run<App: Node + 'static>() {
    let rt = runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let fake_parent: NodeRc = Rc::new(RefCell::new(Box::new(Fake)));
        //render
        {
            App::render(App::new(Rc::downgrade(&fake_parent)));
        }
    });
}
