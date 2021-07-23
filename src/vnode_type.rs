use std::{cell::RefCell, ops::{Deref, DerefMut}, rc::{Rc, Weak}};

use crate::{VTopLevelContainer, VComponent, VContainerWidget, VNode, VTopLevelContainerWidget, VWidget};

pub type StrongNodeType = Rc<RefCell<VNodeType>>;
pub type WeakNodeType = Weak<RefCell<VNodeType>>;

pub enum VNodeType {
    /// User defined container node
    Component(Box<dyn VComponent>),
    /// widget which cannot contain other nodes
    Widget(Box<dyn VWidget>),
    /// widget container which can hild other nodes
    ContainerWidget(Box<dyn VContainerWidget>),
    /// widget which can hold other widgets but can't be added to other widgets
    TopLevelContainerWidget(Box<dyn VTopLevelContainerWidget>),
    /// widget which can hold other widgets but can't be added to other widgets
    TopLevelContainer(Box<dyn VTopLevelContainer>),
}

impl Deref for VNodeType {
    type Target = dyn VNode;

    fn deref(&self) -> &Self::Target {
        match self {
            VNodeType::Component(node) => node.deref().as_ref(),
            VNodeType::Widget(node) => node.deref().as_ref(),
            VNodeType::TopLevelContainerWidget(node) => node.deref().as_ref(),
            VNodeType::TopLevelContainer(node) => node.deref().as_ref(),
            VNodeType::ContainerWidget(node) => node.deref().as_ref(),
        }
    }
}

impl DerefMut for VNodeType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            VNodeType::Component(node) => node.deref_mut().as_mut(),
            VNodeType::Widget(node) => node.deref_mut().as_mut(),
            VNodeType::ContainerWidget(node) => node.deref_mut().as_mut(),
            VNodeType::TopLevelContainer(node) => node.deref_mut().as_mut(),
            VNodeType::TopLevelContainerWidget(node) => node.deref_mut().as_mut(),
        }
    }
}

