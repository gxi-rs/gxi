use std::sync::{Arc, Mutex};

use crate::nodes::component::Component;
use crate::nodes::container::Container;
use crate::nodes::widget::Widget;

pub enum Node {
    Widget(Box<dyn Widget>),
    Container(Box<dyn Container>),
    Component(Box<dyn Component>),
}

pub type AsyncNode = Arc<Mutex<Node>>;

pub trait NodeTrait {
    fn get_child(&self) -> &Option<AsyncNode>;
    fn get_sibling(&self) -> &Option<AsyncNode>;
    fn get_child_mut(&mut self) -> &mut Option<AsyncNode>;
    fn get_sibling_mut(&mut self) -> &mut Option<AsyncNode>;
}
