pub use button::*;
pub use text::*;
pub use view::*;

mod button;
mod text;
mod view;

pub type Node = Option<Box<dyn Component>>;

/// # Component struct :
/// ```rust
/// struct Component {
///     root: T,
///     sibling : Node,
///     child : Node
/// }
/// ```
pub trait Component {
    // getter setter functions
    fn get_sibling(&self) -> &Node {
        unimplemented!()
    }

    fn get_sibling_mut(&mut self) -> &mut Node {
        unimplemented!()
    }

    fn get_child(&self) -> &Node {
        unimplemented!()
    }

    fn get_child_mut(&mut self) -> &mut Node {
        unimplemented!()
    }

    fn render(&mut self, _init: bool) {
        //call backend calls here
    }
}