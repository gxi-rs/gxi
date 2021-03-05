use std::any::Any;
use std::sync::{Arc, Mutex};

use crate::nodes::widget::Widget;

pub type AsyncNode = Arc<Mutex<Box<dyn Node>>>;


pub trait Node {
    fn get_child(&self) -> &Option<AsyncNode>;
    fn get_sibling(&self) -> &Option<AsyncNode>;
    fn get_child_mut(&mut self) -> &mut Option<AsyncNode>;
    fn get_sibling_mut(&mut self) -> &mut Option<AsyncNode>;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_widget(&self) -> &gtk::Widget;
    fn get_widget_as_container(&self) -> &gtk::Container;
}
