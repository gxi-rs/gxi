use std::any::Any;

pub use button::*;
pub use pure::*;
pub use text::*;
pub use view::*;

mod button;
mod text;
mod view;
mod pure;
mod window;

pub type Node = Option<Box<dyn Component>>;
pub type Widget = gtk::Widget;

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
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_widget(&self) -> Option<&Widget> { None }
    fn render(&mut self) { /*call backend calls here */ }
}