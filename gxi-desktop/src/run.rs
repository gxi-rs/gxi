use gxi::{init_member, InitType, GxiNode, Root};

pub fn run<App: GxiNode + 'static>() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        gtk::init().unwrap();
        let root = Root::new_root();
        let app = init_member(root.clone(), InitType::Child, |this| App::new(this)).0;
        App::render(app.into_gxi_node_rc());
        gtk::main();
    });
}
