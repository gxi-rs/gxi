use std::any::Any;

use crate::components::*;
use crate::default_component;

pub struct Button {
    pub widget: gtk::Button,
    pub child: Option<Box<dyn Component>>,
    pub sibling: Option<Box<dyn Component>>,
}

impl Default for Button {
    fn default() -> Self {
        Button {
            widget: Default::default(),
            child: None,
            sibling: None,
        }
    }
}


impl Component for Button {
    default_component!(true);
}