use std::borrow::BorrowMut;
use std::cell::RefCell;

use tokio::task::*;
use tokio::task;

use my_app::MyApp;
use rust_gui::run;

mod my_app;

#[tokio::main]
async fn main() {
    let set = LocalSet::new();
    set.spawn_local(async {
        run::<MyApp>();
        println!("Am i Really Blocking ?")
    });
    set.await;
}
