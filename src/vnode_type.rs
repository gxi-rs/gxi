use std::ops::{Deref, DerefMut};

use crate::{VComponent, VNode, VWidget};

pub enum VNodeType {
    Component(Box<dyn VComponent>),
    Widget(Box<dyn VWidget>),
}

impl Deref for VNodeType {
    type Target = dyn VNode;

    fn deref(&self) -> &Self::Target {
        match self {
            VNodeType::Component(node) => node.deref().as_ref(),
            VNodeType::Widget(node) => node.deref().as_ref()
        }
    }
}

impl DerefMut for VNodeType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            VNodeType::Component(node) => node.deref_mut().as_mut(),
            VNodeType::Widget(node) => node.deref_mut().as_mut()
        }
    }
}
