use crate::components::{Component};

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
}