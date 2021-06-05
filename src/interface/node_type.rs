use crate::{GxiNodeRc, GxiWidgetRc, WeakGxiWidgetRc, WeakGxiNodeRc};
use std::rc::Rc;

#[derive(Clone)]
pub enum NodeType {
    Widget(GxiWidgetRc),
    Component(GxiNodeRc),
}

impl NodeType {
    pub fn into_gxi_node_rc(self) -> GxiNodeRc {
        match self {
            NodeType::Component(comp) => comp,
            NodeType::Widget(widget) => Rc::downcast(widget).unwrap(),
        }
    }
    pub fn downgrade(&self) -> WeakGxiNodeType {
        match self {
            NodeType::Component(this) => WeakGxiNodeType::Component(Rc::downgrade(this)),
            NodeType::Widget(this) => WeakGxiNodeType::Widget(Rc::downgrade(this)),
        }
    }
}

#[derive(Clone)]
pub enum WeakGxiNodeType {
    Widget(WeakGxiWidgetRc),
    Component(WeakGxiNodeRc),
}

impl WeakGxiNodeType {
    pub fn upgrade(self) -> NodeType {
        match self {
            WeakGxiNodeType::Widget(this) => NodeType::Widget(this.upgrade().unwrap()),
            WeakGxiNodeType::Component(this) => NodeType::Component(this.upgrade().unwrap()),
        }
    }
}
