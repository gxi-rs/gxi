use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::{Rc, Weak},
};

use crate::{NativeWidget, VContainer, VLeaf, VNode};

pub type StrongNodeType = Rc<RefCell<Node>>;
pub type WeakNodeType = Weak<RefCell<Node>>;

pub enum Node {
    /// widget which cannot contain other nodes
    Leaf(Box<dyn VLeaf>),
    /// widget which can hold other widgets
    Container(Box<dyn VContainer>),
    /// widget which can hold other widgets but can't be added to other nodes
    TopLevelContainer(Box<dyn VContainer>),
}

impl Into<StrongNodeType> for Node {
    fn into(self) -> StrongNodeType {
        Rc::new(RefCell::new(self))
    }
}

impl Deref for Node {
    type Target = dyn VNode;

    fn deref(&self) -> &Self::Target {
        match self {
            Node::Leaf(node) => node.deref().as_ref(),
            Node::TopLevelContainer(node) => node.deref().as_ref(),
            Node::Container(node) => node.deref().as_ref(),
        }
    }
}

impl DerefMut for Node {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Node::Leaf(node) => node.deref_mut().as_mut(),
            Node::Container(node) => node.deref_mut().as_mut(),
            Node::TopLevelContainer(node) => node.deref_mut().as_mut(),
        }
    }
}

impl Node {
    pub fn get_native_widget(&self) -> &NativeWidget {
        match self {
            Node::Leaf(w) => w.deref(),
            Node::Container(w) => w.deref(),
            Node::TopLevelContainer(w) => w.deref(),
        }
    }
}
