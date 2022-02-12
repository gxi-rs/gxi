use std::{cell::RefCell, ops::Deref};

use crate::{Observer, Observers};

pub struct Observable<V> {
    value: RefCell<V>,
    observers: RefCell<Observers<RefCell<V>>>,
}

impl<V> Deref for Observable<V> {
    type Target = RefCell<V>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<V> From<V> for Observable<V> {
    fn from(v: V) -> Self {
        Self {
            observers: Default::default(),
            value: RefCell::new(v),
        }
    }
}

impl<V> Observable<V> {
    /// add observer which is called when value under Observable changes
    pub fn add_observer(&self, mut observer: Observer<RefCell<V>>) {
        (*observer)(&self.value);
        self.observers.borrow_mut().push(observer);
    }

    /// notifies all observers that the value has changed
    pub fn notify(&self) {
        self.observers.borrow_mut().notify(&self.value);
    }
}
