use std::any::Any;

use gtk::ButtonExt;

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

impl Button {
    pub fn on_click<F: Fn() + 'static>(&self, f: F) {
        self.widget.connect_clicked(move |_| f());
    }
    pub fn set_label(&self, label: &str) {
        self.widget.set_label(label);
    }
}