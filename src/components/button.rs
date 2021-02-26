use std::any::Any;

use gtk::{ButtonExt, CheckButton};

use crate::components::{Component, Node, Widget};

pub struct Button {
    pub widget: CheckButton,
    pub child: Option<Box<dyn Component>>,
    pub sibling: Option<Box<dyn Component>>,
    pub label: String,
}

impl Default for Button {
    fn default() -> Self {
        Button {
            widget: Default::default(),
            child: None,
            sibling: None,
            label: String::new(),
        }
    }
}

impl Component for Button {
    fn get_sibling(&self) -> &Node { &self.sibling }

    fn get_sibling_mut(&mut self) -> &mut Node {
        &mut self.sibling
    }

    fn get_child(&self) -> &Node { &self.child }

    fn get_child_mut(&mut self) -> &mut Node {
        &mut self.child
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_widget(&self) -> Option<&Widget> {
        Some(self.widget.as_ref())
    }

    fn render(&mut self) {
        self.widget.set_label(self.label.as_str());
    }
}