pub use h1::*;
pub use h2::*;
pub use h3::*;
pub use h4::*;
pub use h5::*;
pub use h6::*;
pub use p::*;
use rust_gui_interface::Node;

mod h1;
mod h2;
mod h3;
mod h4;
mod h5;
mod h6;
mod p;

pub trait TextExt: Node {
    fn label(&self, text: &str) {
        self.get_widget().set_inner_html(&text);
    }
}