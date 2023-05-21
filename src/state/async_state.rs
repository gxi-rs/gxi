use std::{
    ops::Deref,
    sync::{Arc, Weak},
};

use crate::AsyncObservable;

#[derive(Clone)]
pub struct AsyncState<V>(Arc<AsyncObservable<V>>);

#[derive(Clone)]
pub struct WeakAsyncState<V>(Weak<AsyncObservable<V>>);

impl<V> From<V> for AsyncState<V> {
    fn from(v: V) -> Self {
        Self(Arc::new(AsyncObservable::from(v)))
    }
}

impl<V> AsyncState<V> {
    pub fn downgrade(&self) -> WeakAsyncState<V> {
        WeakAsyncState(Arc::downgrade(&self.0))
    }
}

impl<V> Deref for AsyncState<V> {
    type Target = AsyncObservable<V>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<V> Drop for AsyncState<V> {
    fn drop(&mut self) {
        (*self).notify();
    }
}

impl<V> WeakAsyncState<V> {
    pub fn upgrade(&self) -> AsyncState<V> {
        AsyncState(self.0.upgrade().unwrap())
    }
}
