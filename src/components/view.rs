use std::any::Any;

use gtk::{ContainerExt, Grid, WidgetExt};

use crate::components::*;
use crate::default_component;

pub struct View {
    pub widget: gtk::ListBox,
    pub sibling: Node,
    pub child: Node,
    pub traversed: bool,
}

unsafe impl Send for View {}

unsafe impl Sync for View {}

impl Default for View {
    fn default() -> Self {
        View {
            widget: Default::default(),
            sibling: None,
            child: None,
            traversed: false,
        }
    }
}

impl Component for View {
    default_component!(true);

    fn render(&mut self) {
        if !self.traversed {
            match self.child.as_ref() {
                Some(node) => {
                    traverse(&mut self.widget, node);
                    self.traversed = true;
                }
                _ => {}
            };
        }
    }
}

fn traverse<T: ContainerExt>(widget: &mut T, node: &Box<dyn Component>) {
    match node.get_widget() {
        Some(w) => {
            widget.add(w);
        }
        //pure and custom components
        None => {
            match node.get_child() {
                Some(s) => { traverse(widget, s) }
                _ => {}
            }
        }
    };
    match node.get_sibling().as_ref() {
        Some(s) => traverse(widget, s),
        _ => return
    };
}
