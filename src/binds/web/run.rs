use std::cell::RefCell;
use std::panic;

use crate::*;

thread_local! {
    static TREE_ROOT: RefCell<Option<StrongNodeType>> = RefCell::new(None);
}

pub fn run<App: Node + 'static>() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    TREE_ROOT.with(|root| {
        let tree = Root::new_root();
        let (app, ..) = init_member(tree.clone(), InitType::Child, |this| App::new(this), true);
        App::render(app);
        root.borrow_mut().replace(tree);
    });
}
