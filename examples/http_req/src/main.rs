use app::App;
use rust_gui::*;

mod app;

fn main() {
    let mut rt = runtime::Runtime::new().unwrap();
    rt.block_on(async {
        run::<App>();
    });
}
