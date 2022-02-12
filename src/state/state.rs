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
    pub fn downgrade(&self) -> WeakState<V> {
        WeakState(Rc::downgrade(&self.0))
    }
}

impl<V> From<V> for State<V> {
    fn from(v: V) -> Self {
        Self(Rc::new(Observable::from(v)))
    }
}

impl<V> Deref for State<V> {
    type Target = Observable<V>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<V> WeakState<V> {
    pub fn upgrade(&self) -> Option<State<V>> {
        self.0.upgrade().map(|v| State(v))
    }
}

pub fn add_multi_observer<V>(state: &State<V>, multi_observer: WeakState<()>) {
    state.add_observer(Box::new(move |_| {
        if let Some(multi_observer) = multi_observer.upgrade() {
            multi_observer.notify();
            false
        } else {
            true
        }
    }));
}
