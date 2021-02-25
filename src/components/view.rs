use crate::components::{Component, ComponentType, Node};

pub struct View {
    pub sibling: Option<Box<dyn Component>>,
    pub child: Option<Box<dyn Component>>,
}

impl Default for View {
    fn default() -> Self {
        View {
            sibling: None,
            child: None,
        }
    }
}

impl Component for View {
    fn get_sibling(&self) -> &Node { &self.sibling }

    fn get_sibling_mut(&mut self) -> &mut Node {
        &mut self.sibling
    }

    fn get_child(&self) -> &Node { &self.child }

    fn get_child_mut(&mut self) -> &mut Node {
        &mut self.child
    }

    fn get_type(&self) -> ComponentType {
        ComponentType::View
    }
}