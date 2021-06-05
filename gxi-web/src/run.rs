use std::cell::RefCell;
use std::panic;
use std::rc::Rc;

use crate::*;

thread_local! {
    static TREE_ROOT: RefCell<Option<NodeRc>> = RefCell::new(None);
}

pub fn run<App: Node + 'static>() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    TREE_ROOT.with(|root| {
        let tree = Root::new_root();
        //render
        {
            let app = {
                let tree_weak_clone = Rc::downgrade(&tree);
                tree.borrow_mut()
                    .init_child(Box::new(|| App::new(tree_weak_clone)))
                    .0
            };
            App::render(app);
        }
        root.borrow_mut().replace(tree);
    });
}
