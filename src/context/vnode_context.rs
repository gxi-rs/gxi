use std::{any::Any, rc::Rc};

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
