use crate::components::{Component, ComponentType, Node};
use std::any::Any;

pub struct Text {
    pub label: String,
    pub child: Option<Box<dyn Component>>,
    pub sibling: Option<Box<dyn Component>>,
}

impl Default for Text {
    fn default() -> Self {
        Text {
            label: String::new(),
            child: None,
            sibling: None,
        }
    }
}

impl Component for Text {
    fn get_sibling(&self) -> &Node { &self.sibling }

    fn get_sibling_mut(&mut self) -> &mut Node {
        &mut self.sibling
    }

    fn get_child(&self) -> &Node { &self.child }

    fn get_child_mut(&mut self) -> &mut Node {
        &mut self.child
    }

    fn get_type(&self) -> ComponentType {
        ComponentType::Text
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}