use std::sync::{Arc, Mutex};

use crate::nodes::component::Component;
use crate::nodes::container::Container;
use crate::nodes::widget::Widget;

pub enum Node {
    Widget(Box<dyn Widget>),
    Container(Box<dyn Container>),
    Component(Box<dyn Component>),
}

pub trait NodeTrait {
    fn get_child(&self) -> &Option<Arc<Mutex<Node>>>;
    fn get_sibling(&self) -> &Option<Arc<Mutex<Node>>>;
    fn get_child_mut(&mut self) -> &mut Option<Arc<Mutex<Node>>>;
    fn get_sibling_mut(&mut self) -> &mut Option<Arc<Mutex<Node>>>;
}
