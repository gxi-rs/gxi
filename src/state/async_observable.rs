use std::{ops::Deref, sync::Mutex};

use crate::{Observer, Observers};

pub struct AsyncObservable<V> {
    value: Mutex<V>,
    observers: Mutex<Observers<V>>,
}

impl<V> Deref for AsyncObservable<V> {
    type Target = Mutex<V>;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<V> From<V> for AsyncObservable<V> {
    fn from(v: V) -> Self {
        Self {
            observers: Default::default(),
            value: Mutex::new(v),
        }
    }
}

impl<V> AsyncObservable<V> {
    /// add observer which is called when value under AsyncObservable changes
    pub fn add_observer(&self, observer: Observer<V>) {
        self.observers.lock().unwrap().push(observer);
    }

    /// notifies all observers that the value has changed
    pub fn notify(&self) {
        self.observers
            .lock()
            .unwrap()
            .notify(&self.value.lock().unwrap());
    }
}
