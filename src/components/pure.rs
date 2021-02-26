use std::any::Any;

use crate::components::*;
use crate::default_component;

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
    default_component!(false);
}