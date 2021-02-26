use std::any::Any;

use gtk::{ContainerExt, Grid};

use crate::components::{Component, Node, Widget};

pub struct View {
    pub widget: Grid,
    pub sibling: Node,
    pub child: Node,
}

impl Default for View {
    fn default() -> Self {
        View {
            widget: Default::default(),
            sibling: None,
            child: None,
        }
    }
}

impl Component for View {
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
        match self.child.as_ref() {
            Some(node) => {
                traverse(&mut self.widget, node);
            }
            _ => {}
        };
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
