use std::rc::Rc;

use crate::{*};

pub fn run<App: Node + 'static>() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        gtk::init().unwrap();
        let root = Root::new_root();
        //render
        let root_weak_rc = Rc::downgrade(&root);
        let app = root.borrow_mut().init_child(Box::new(|| App::new(root_weak_rc))).0;
        App::render(app.clone());
        //start main loop
        gtk::main();
    });
}
