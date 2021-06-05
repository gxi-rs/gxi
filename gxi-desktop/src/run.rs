use std::rc::Rc;
use gxi::{Node, Root, init_member, InitType};
use std::borrow::BorrowMut;

pub fn run<App: Node + 'static>() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        gtk::init().unwrap();
        let root = Root::new_root();
        let app = init_member(root.clone(), InitType::Child, |this| App::new(this)).0;
        App::render(app.into_gxi_node_rc());
        gtk::main();
    });
}
