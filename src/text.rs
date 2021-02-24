use crate::component::{Component, Nodes};

pub struct Text {
    pub label: String
}

impl Default for Text {
    fn default() -> Self {
        Text {
            label: String::new()
        }
    }
}

impl Component for Text {
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