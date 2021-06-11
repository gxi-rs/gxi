use crate::{init_member, InitType, Node, Root, Window};
use std::rc::Rc;

pub fn run<App: Node + 'static>() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        gtk::init().unwrap();
        let root = Root::new_root();
        let root = Window::new(Rc::downgrade(&root));
        let app = init_member(root.clone(), InitType::Child, |this| App::new(this)).0;
        App::render(app);
        gtk::main();
    });
}
