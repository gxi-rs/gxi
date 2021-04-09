use my_app::MyApp;
use rust_gui::*;

mod my_app;

fn main() {
    let mut rt = runtime::Runtime::new().unwrap();
    rt.block_on(async {
        run::<MyApp>();
    });
}
