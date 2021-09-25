use std::{cell::RefCell, ops::Deref, rc::Rc};

pub type Observer<V> = Box<dyn FnMut(&V) -> ()>;

#[derive(Clone)]
pub struct Observable<V> {
    value: Rc<RefCell<V>>,
    observers: Rc<RefCell<Vec<Observer<V>>>>,
    value_synced: bool,
}

impl<V> Deref for Observable<V> {
    type Target = RefCell<V>;

    fn deref(&self) -> &Self::Target {
        &*self.value
    }
}

impl<V> Observable<V> {
    pub fn new(value: V) -> Self {
        Self {
            observers: Default::default(),
            value: Rc::new(RefCell::new(value)),
            value_synced: false,
        }
    }

    /// mutates value, and notifies all the observers
    pub fn set_value(&mut self, value: V) {
        *self.value.as_ref().borrow_mut() = value;
        self.notify();
    }

    /// add observer which is called when value under Observable changes
    pub fn add_observer(&mut self, observer: Observer<V>) {
        self.observers.as_ref().borrow_mut().push(observer);
    }

    /// notifies all observers that the value has changed
    pub fn notify(&self) {
        if !self.value_synced {
            let mut observers = self.observers.as_ref().borrow_mut();
            for observer in &mut *observers {
                (observer)(&*self.value.as_ref().borrow_mut());
            }
        }
    }
}

impl<V> Drop for Observable<V> {
    fn drop(&mut self) {
        self.notify();
    }
}
impl<V> From<V> for Observable<V> {
    fn from(v: V) -> Self {
        Self::new(v)
    }
}
