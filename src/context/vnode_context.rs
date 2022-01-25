use std::{rc::Rc, any::Any};

use crate::VNode;

/// returned by components
/// holds VNode
pub struct VNodeContext {
    pub vnode: VNodeShell,
    pub ctx: Option<Box<dyn Any>>,
}

pub enum VNodeShell {
    Default(Box<dyn VNode>),
    Rc(Rc<dyn VNode>),
}

impl VNodeContext {
    pub fn from(vnode: VNodeShell, ctx: Option<Box<dyn Any>>) -> Self {
        Self { vnode, ctx }
    }
}
