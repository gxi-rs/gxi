use std::rc::Rc;

use crate::{GxiComponentRc, GxiContainerRc, GxiNodeRc, GxiWidgetRc, WeakGxiWidgetRc, WeakGxiContainerRc, WeakGxiComponentRc};

#[derive(Clone)]
pub enum GxiNodeType {
    Widget(GxiWidgetRc),
    Component(GxiComponentRc),
    Container(GxiContainerRc),
}

impl GxiNodeType {
    pub fn into_gxi_node_rc(self) -> GxiNodeRc {
        match self {
            GxiNodeType::Widget(this) => Rc::downcast(this).unwrap(),
            GxiNodeType::Component(this) => Rc::downcast(this).unwrap(),
            GxiNodeType::Container(this) => Rc::downcast(this).unwrap(),
        }
    }
    pub fn downgrade(&self) -> WeakGxiNodeType {
        match &self {
            GxiNodeType::Widget(this) => WeakGxiNodeType::Widget(Rc::downgrade(this)),
            GxiNodeType::Component(this) => WeakGxiNodeType::Component(Rc::downgrade(this)),
            GxiNodeType::Container(this) => WeakGxiNodeType::Container(Rc::downgrade(this))
        }
    }
}

#[derive(Clone)]
pub enum WeakGxiNodeType {
    Widget(WeakGxiWidgetRc),
    Component(WeakGxiComponentRc),
    Container(WeakGxiContainerRc),
}

impl WeakGxiNodeType {
    pub fn upgrade(self) -> GxiNodeType {
        match self {
            WeakGxiNodeType::Widget(this) => GxiNodeType::Widget(this.upgrade().unwrap()),
            WeakGxiNodeType::Container(this) => GxiNodeType::Container(this.upgrade().unwrap()),
            WeakGxiNodeType::Component(this) => GxiNodeType::Component(this.upgrade().unwrap()),
        }
    }
}
