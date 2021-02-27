use std::any::Any;
use std::cell::{RefCell, RefMut};

use gtk::ButtonExt;

use crate::{default_component, MyApp, MyAppState};
use crate::components::*;

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
    pub fn on_click(&self, f: Box<dyn Fn()>) {
        self.widget.connect_clicked(move |_| f());
    }
    pub fn set_label(&self, label: &str) {
        self.widget.set_label(label);
    }
}