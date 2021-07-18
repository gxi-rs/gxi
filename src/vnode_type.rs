use std::ops::{Deref, DerefMut};

use crate::{VComponent, VNode, VWidget, VTopLevelWidget};

pub enum VNodeType {
    /// User defined Node
    Component(Box<dyn VComponent>),
    /// widget which may or may not contain other nodes
    Widget(Box<dyn VWidget>),
    /// widget which can hold other widgets but can't be added to other widgets
    TopLevelWidget(Box<dyn VTopLevelWidget>)
}

impl Deref for VNodeType {
    type Target = dyn VNode;

    fn deref(&self) -> &Self::Target {
        match self {
            VNodeType::Component(node) => node.deref().as_ref(),
            VNodeType::Widget(node) => node.deref().as_ref(),
            VNodeType::TopLevelWidget(node) => node.deref().as_ref(),
        }
    }
}

impl DerefMut for VNodeType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            VNodeType::Component(node) => node.deref_mut().as_mut(),
            VNodeType::Widget(node) => node.deref_mut().as_mut(),
            VNodeType::TopLevelWidget(node) => node.deref_mut().as_mut(),
        }
    }
}

