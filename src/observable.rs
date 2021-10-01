use std::{cell::RefCell, ops::Deref};

/// bool: true -> remove observer
pub type Observer<V> = Box<dyn FnMut(&V) -> bool>;

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
        let mut to_remove = vec![];

        {
            let mut x = 0usize;
            for observer in &mut *observers {
                if (observer)(&*self.value.borrow_mut()) {
                    to_remove.push(x);
                } else {
                    x += 1;
                }
            }
        }

        for x in to_remove {
            drop(observers.remove(x));
        }
    }
}

impl<V> From<V> for Observable<V> {
    fn from(v: V) -> Self {
        Self::new(v)
    }
}
