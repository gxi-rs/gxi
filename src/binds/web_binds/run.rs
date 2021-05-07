use std::cell::RefCell;
use std::panic;
use std::rc::Rc;

use crate::{*};

thread_local! {
    static TREE_ROOT: RefCell<Option<NodeRc>> = RefCell::new(None);
}

pub fn run<App: Node + 'static>() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    TREE_ROOT.with(|root| {
        let fake_parent: NodeRc = Rc::new(RefCell::new(Box::new(Fake)));
        let body = Body::new(Rc::downgrade(&fake_parent));
        //render
        let app = {
            let body_clone = Rc::downgrade(&body);
            body.borrow_mut()
                .init_child(Box::new(|| App::new(body_clone)))
                .0
        };
        App::render(app);
        root.borrow_mut().replace(body);
    });
}
