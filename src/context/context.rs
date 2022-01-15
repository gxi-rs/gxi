use crate::MemDump;
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

#[derive(Default)]
pub struct ContextNodeInner {
    pub dump: RefCell<MemDump>,
    pub parent: StrongContextNode,
}

#[derive(Default)]
pub struct ContextNode(ContextNodeInner);

impl Deref for ContextNode {
    type Target = ContextNodeInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ContextNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for ContextNodeInner {
    fn drop(&mut self) {
        self.parent
    }
}
