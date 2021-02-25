use std::any::Any;

use crate::components::{Component, Node};

pub struct Button {
    pub child: Option<Box<dyn Component>>,
    pub sibling: Option<Box<dyn Component>>,
    pub label: String,
}

impl Default for Button {
    fn default() -> Self {
        println!("Button init");
        Button {
            child: None,
            sibling: None,
            label: String::new(),
        }
    }
}

impl Component for Button {
    fn get_sibling(&self) -> &Node { &self.sibling }

    fn get_sibling_mut(&mut self) -> &mut Node {
        &mut self.sibling
    }

    fn get_child(&self) -> &Node { &self.child }

    fn get_child_mut(&mut self) -> &mut Node {
        &mut self.child
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn render(&mut self) {
        println!("({})", self.label);
    }
}