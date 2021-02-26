use std::any::Any;

use gtk::Label;

use crate::components::*;
use crate::default_component;

pub struct Text {
    pub widget: Label,
    pub child: Option<Box<dyn Component>>,
    pub sibling: Option<Box<dyn Component>>,
}

impl Default for Text {
    fn default() -> Self {
        Text {
            widget: Label::new(None),
            child: None,
            sibling: None,
        }
    }
}

impl Component for Text {
    default_component!(true);
}