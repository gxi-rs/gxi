use std::any::Any;

use gtk::WindowType;

use crate::components::*;
use crate::default_component;

pub struct Window {
    pub widget: gtk::Window,
    pub child: Option<Box<dyn Component>>,
    pub sibling: Option<Box<dyn Component>>,
    pub label: String,
}
unsafe impl Send for Window {}
unsafe impl Sync for Window {}

impl Default for Window {
    fn default() -> Self {
        Window {
            widget: gtk::Window::new(WindowType::Toplevel),
            child: None,
            sibling: None,
            label: String::new(),
        }
    }
}

impl Component for Window {
    default_component!(true);
}