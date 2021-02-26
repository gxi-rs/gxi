use std::any::Any;

use gtk::{Label, LabelExt};

use crate::components::{Component, Node, Widget};

pub struct Text {
    pub widget: Label,
    pub label: String,
    pub child: Option<Box<dyn Component>>,
    pub sibling: Option<Box<dyn Component>>,
}

impl Default for Text {
    fn default() -> Self {
        Text {
            widget: Label::new(None),
            label: String::new(),
            child: None,
            sibling: None,
        }
    }
}

impl Component for Text {
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
        self.widget.set_label(self.label.as_str())
    }
}