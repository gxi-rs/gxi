use crate::components::Component;

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

impl Component for Text {}