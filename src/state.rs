use std::{
    ops::Deref,
    rc::{Rc, Weak},
};

use crate::Observable;

#[derive(Clone)]
pub struct State<V>(Rc<Observable<V>>);

#[derive(Clone)]
pub struct WeakState<V>(Weak<Observable<V>>);

impl<V> State<V> {
    pub fn new(v: V) -> Self {
        Self(Rc::new(Observable::new(v)))
    }

    pub fn downgrade(&self) -> WeakState<V> {
        WeakState(Rc::downgrade(&self.0))
    }
}

impl<V> Deref for State<V> {
    type Target = Observable<V>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<V> Drop for State<V> {
    fn drop(&mut self) {
        (*self).notify();
    }
}

impl<V> From<V> for State<V> {
    fn from(v: V) -> Self {
        Self::new(v)
    }
}

impl<V> WeakState<V> {
    pub fn upgrade(&self) -> State<V> {
        State(self.0.upgrade().unwrap())
    }
}
