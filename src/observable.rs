use std::ops::{Deref, DerefMut};

pub type Observer<V> = Box<dyn FnMut(&V) -> ()>;

pub struct Observable<V> {
    observers: Vec<Observer<V>>,
    value: V,
    /// true if all observers have been notified about the change
    synced: bool,
}

impl<V> Deref for Observable<V> {
    type Target = V;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<V> DerefMut for Observable<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.synced = false;
        &mut self.value
    }
}

impl<V> Observable<V> {
    pub fn new(value: V) -> Self {
        Self {
            observers: Default::default(),
            value,
            synced: true,
        }
    }

    /// mutates value, and notifies all the observers
    pub fn set_value(&mut self, value: V) {
        **self = value;
        self.notify();
    }

    /// add observer which is called when value under Observable changes
    pub fn add_observer(&mut self, observer: Observer<V>) {
        self.observers.push(observer);
    }

    /// notifies all observers that the value has changed
    pub fn notify(&mut self) {
        if !self.synced {
            for observer in &mut self.observers {
                (observer)(&self.value);
            }
        }
    }
}

impl<V> From<V> for Observable<V> {
    fn from(v: V) -> Self {
        Self::new(v)
    }
}
