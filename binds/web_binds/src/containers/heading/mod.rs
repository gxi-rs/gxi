pub use h1::*;
pub use h2::*;
use rust_gui_interface::Node;

mod h1;
mod h2;

pub trait Heading:Node {
    fn label(&self, text: &str) {
        self.get_widget().set_inner_html(&text);
    }
}