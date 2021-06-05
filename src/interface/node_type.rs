use crate::{GxiNodeRc, GxiWidgetRc, WeakGxiNodeRc, WeakGxiWidgetRc};
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
    pub fn downgrade(&self) -> WeakNodeType {
        match self {
            NodeType::Component(this) => WeakNodeType::Component(Rc::downgrade(this)),
            NodeType::Widget(this) => WeakNodeType::Widget(Rc::downgrade(this)),
        }
    }
}

#[derive(Clone)]
pub enum WeakNodeType {
    Widget(WeakGxiWidgetRc),
    Component(WeakGxiNodeRc),
}

impl WeakNodeType {
    pub fn upgrade(self) -> NodeType {
        match self {
            WeakNodeType::Widget(this) => NodeType::Widget(this.upgrade().unwrap()),
            WeakNodeType::Component(this) => NodeType::Component(this.upgrade().unwrap()),
        }
    }
}
