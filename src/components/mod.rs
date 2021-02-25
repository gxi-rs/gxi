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
    fn get_sibling(&self) -> &Node;
    fn get_sibling_mut(&mut self) -> &mut Node;
    fn get_child(&self) -> &Node;
    fn get_child_mut(&mut self) -> &mut Node;
    fn render(&mut self) {
        //call backend calls here
    }
}