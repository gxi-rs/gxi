use std::cell::RefCell;
use std::rc::Rc;

use rust_gui_interface::{*};
use std::panic;
use crate::Body;

pub fn run<App: Node + 'static>() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let fake_parent: NodeRc = Rc::new(RefCell::new(Box::new(Fake)));
    let body = Body::new(Rc::downgrade(&fake_parent));
    //render
    {
        App::render(App::new(Rc::downgrade(&body)));
    }
    /*let rt = runtime::Runtime::new().unwrap();
    rt.block_on(async {

    });*/
}
