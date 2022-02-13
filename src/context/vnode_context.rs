use std::{any::Any, ops::Deref, rc::Rc};

/// returned by components
/// holds VNode
pub enum VNodeContext<T> {
    NoCtx(VNodeShell<T>),
    WithCtx(VNodeShell<T>, Box<dyn Any>),
}

pub enum VNodeShell<T> {
    Default(T),
    Rc(Rc<T>),
}

impl<T> Deref for VNodeShell<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            VNodeShell::Default(t) => t,
            VNodeShell::Rc(t) => &*t,
        }
    }
}

impl<T> Deref for VNodeContext<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            VNodeContext::NoCtx(i) => &i,
            VNodeContext::WithCtx(i, _) => &i,
        }
    }
}
