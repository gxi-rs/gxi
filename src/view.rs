use std::collections::HashMap;

use crate::component::{Component, Nodes};

pub struct View {
    nodes: HashMap<i32, Box<dyn Component>>
}

impl Default for View {
    fn default() -> Self {
        View { nodes: Default::default() }
    }
}

impl Component for View {
    fn get_nodes(&self) -> Option<&Nodes> {
        Some(&self.nodes)
    }

    fn get_mut_nodes(&mut self) -> Option<&mut Nodes> {
        Some(&mut self.nodes)
    }

    fn render(&self, _tree: &mut Nodes, _first: bool) {
        unimplemented!()
    }
}