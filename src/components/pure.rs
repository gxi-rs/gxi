use std::any::Any;

use crate::components::{Component, Node};

pub struct Pure {
    pub sibling: Option<Box<dyn Component>>,
    pub child: Option<Box<dyn Component>>,
    pub type_extra: i32,
}

impl Default for Pure {
    fn default() -> Self {
        Pure {
            sibling: None,
            child: None,
            type_extra: -1,
        }
    }
}

impl Component for Pure {
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
}