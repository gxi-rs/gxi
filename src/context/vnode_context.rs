use std::any::Any;

/// returned by components
/// holds VNode
pub struct VNodeContext<T> {
    pub vnode: VNodeShell<T>,
    pub ctx: Option<Box<dyn Any>>,
}

pub enum VNodeShell<T> {
    Default(T),
    Rc(T),
}

impl<T> VNodeContext<T> {
    pub fn from(vnode: VNodeShell<T>, ctx: Option<Box<dyn Any>>) -> Self {
        Self { vnode, ctx }
    }
}
