use crate::component::{Component, Nodes};

pub struct Button {
    pub label: String
}

impl Default for Button {
    fn default() -> Self {
        Button {
            label: String::new()
        }
    }
}

impl Component for Button {
    fn get_nodes(&self) -> Option<&Nodes> {
        None
    }

    fn get_mut_nodes(&mut self) -> Option<&mut Nodes> {
        None
    }

    fn render(&self, _tree: &mut Nodes, _first: bool) {
        unimplemented!()
    }
}