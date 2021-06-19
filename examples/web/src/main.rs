use app::*;
use cat_fact::*;
use counter::*;
use gxi::*;
use todo::*;

mod app;
mod cat_fact;
mod counter;
mod todo;

fn main() {
    run::<App>();
}
