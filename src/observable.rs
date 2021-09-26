use std::{cell::RefCell, ops::Deref};

pub type Observer<V> = Box<dyn FnMut(&V) -> ()>;

pub struct Observable<V> {
    value: RefCell<V>,
    observers: RefCell<Vec<Observer<V>>>,
}

impl<V> Deref for Observable<V> {
    type Target = RefCell<V>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<V> Observable<V> {
    pub fn new(value: V) -> Self {
        Self {
            observers: Default::default(),
            value: RefCell::new(value),
        }
    }

    /// add observer which is called when value under Observable changes
    pub fn add_observer(&self, observer: Observer<V>) {
        self.observers.borrow_mut().push(observer);
    }

    /// notifies all observers that the value has changed
    pub fn notify(&self) {
        let mut observers = self.observers.borrow_mut();
        for observer in &mut *observers {
            (observer)(&*self.value.borrow_mut());
        }
    }
}

impl<V> From<V> for Observable<V> {
    fn from(v: V) -> Self {
        Self::new(v)
    }
}
