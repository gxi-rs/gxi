use std::rc::Rc;
use gxi::{Node, Root, init_member, InitType};
use std::borrow::BorrowMut;

pub fn run<App: Node + 'static>() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        gtk::init().unwrap();
        let root = Root::new_root();
        init_member(root.clone(), InitType::Child, |this| App::new(this, ()));

        //render
        let root_weak_rc = Rc::downgrade(&root);
        let app = root
            .borrow_mut()
            .init_child(Box::new(|| App::new(root_weak_rc)))
            .0;
        App::render(app.clone());
        //start main loop
        gtk::main();
    });
}
