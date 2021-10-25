mod app;
mod cat_fact;
mod counter;

pub use cat_fact::*;
pub use counter::*;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    gxi::run(unsafe { app::app() });
}
