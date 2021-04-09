use my_app::MyApp;
use rust_gui::run;
use tokio_global::Runtime;

mod my_app;

fn main() {
    let _guard = Runtime::default();

    let runner = std::thread::spawn(|| {
        Runtime::run();
    });
    run::<MyApp>();
}
