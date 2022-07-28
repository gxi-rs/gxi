use crate::{MemDump, VNodeContext, VNodeShell};

/// A RefCell wrapper around MemDump
/// Used for nodes which live throughout the
/// lifetime of the component
pub type ConstContext = MemDump;

impl ConstContext {
    /// absorb ctx from vnode context and return vnode shell from it
    pub fn absorb<T: 'static>(&mut self, node: VNodeContext<T>) -> crate::VNodeShell<T> {
        match node {
            VNodeContext::NoCtx(t) => t,
            VNodeContext::WithCtx(t, ctx) => {
                self.push(ctx);
                if let VNodeShell::Rc(t) = &t {
                    self.push(Box::new(t.clone()))
                }
                t
            }
        }
    }
}
