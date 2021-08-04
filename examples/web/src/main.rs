mod app;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    gxi::run::<app::App>();
}
