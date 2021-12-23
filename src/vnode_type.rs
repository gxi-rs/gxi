use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
};

use crate::{NativeWidget, VContainerWidget, VNode, VWidget};

pub type StrongNodeType = Rc<RefCell<VNodeType>>;
pub type WeakNodeType = Weak<RefCell<VNodeType>>;

pub enum VNodeType {
    /// widget which cannot contain other nodes
    Widget(Box<dyn VWidget>),
    /// widget which can hold other widgets
    ContainerWidget(Box<dyn VContainerWidget>),
    /// widget which can hold other widgets but can't be added to other nodes
    TopLevelContainerWidget(Box<dyn VContainerWidget>),
}

impl Deref for VNodeType {
    type Target = dyn VNode;

    fn deref(&self) -> &Self::Target {
        match self {
            VNodeType::Widget(node) => node.deref().as_ref(),
            VNodeType::TopLevelContainerWidget(node) => node.deref().as_ref(),
            VNodeType::ContainerWidget(node) => node.deref().as_ref(),
        }
    }
}

impl DerefMut for VNodeType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            VNodeType::Widget(node) => node.deref_mut().as_mut(),
            VNodeType::ContainerWidget(node) => node.deref_mut().as_mut(),
            VNodeType::TopLevelContainerWidget(node) => node.deref_mut().as_mut(),
        }
    }
}

impl VNodeType {
    pub fn get_native_widget(&self) -> &NativeWidget {
        match self {
            VNodeType::Widget(w) => w.deref(),
            VNodeType::ContainerWidget(w) => w.deref(),
            VNodeType::TopLevelContainerWidget(w) => w.deref(),
        }
    }
}
