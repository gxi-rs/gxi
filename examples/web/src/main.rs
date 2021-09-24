mod app;
//mod counter;

//pub(crate) use counter::*;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    gxi::run(app::app());
}
