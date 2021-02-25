pub use button::*;
pub use text::*;
pub use view::*;

mod button;
mod text;
mod view;

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
    fn get_sibling(&self) -> Option<&Box<dyn Component>> {
        None
    }

    fn get_sibling_mut(&mut self) -> Option<&mut Box<dyn Component>> {
        None
    }

    fn get_child(&self) -> Option<&Box<dyn Component>> {
        None
    }

    fn get_child_mut(&mut self) -> Option<&mut Box<dyn Component>> {
        None
    }

    fn render(&mut self, _init: bool) {
        //call backend calls here
    }
}