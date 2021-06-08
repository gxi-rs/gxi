//pub use app::*;
//pub use centre::*;
//pub use counter::*;
pub use gxi_desktop::*;
pub use gxi_desktop::gxi::*;
pub use gxi::*;

//mod app;
//mod centre;
//mod counter;

gxi! {
    App {}
    render {
        Window [
       //     View [
         //   ]
       ]
    }
}

fn main() {
    run::<App>();
}
