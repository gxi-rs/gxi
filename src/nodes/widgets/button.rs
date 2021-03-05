use std::ops::DerefMut;

use gtk::ContainerExt;

use crate::nodes::node::{AsyncNode, Node, NodeTrait};
use crate::nodes::widget::Widget;

pub struct Button {
    pub widget: gtk::Button,
    pub child: Option<AsyncNode>,
    pub sibling: Option<AsyncNode>,
    pub parent: AsyncNode,
}

impl NodeTrait for Button {
    impl_node_trait!();
}

impl Drop for Button {
    fn drop(&mut self) {
        if let Node::Container(s) = self.parent.as_ref().lock().unwrap().deref_mut() {
            s.get_widget().remove(&self.widget);
        }
    }
}

impl Widget for Button {}

impl Button {
    pub fn new(parent: AsyncNode) -> Self {
        Button {
            widget: gtk::Button::new(),
            child: None,
            sibling: None,
            parent,
        }
    }
}
