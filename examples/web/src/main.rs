mod app;
mod cat_fact;
mod counter;
mod todo;

pub use cat_fact::*;
pub use counter::*;
pub use todo::*;

fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    gxi::run(app::app());
}
