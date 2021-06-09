pub use app::*;
pub use centre::*;
pub use counter::*;
pub use gxi::*;

mod app;
mod centre;
mod counter;

fn main() {
    run::<App>();
}
