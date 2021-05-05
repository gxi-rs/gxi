pub use app::App;
pub use gxi::*;
pub use counter::Counter;

mod app;
mod centre;
mod counter;

fn main() {
    run::<App>();
}
