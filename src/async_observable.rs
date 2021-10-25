use std::{
    ops::Deref,
    sync::{Mutex, MutexGuard},
};

/// bool: true -> remove observer
pub type Observer<V> = Box<dyn FnMut(&V) -> bool>;

pub struct AsyncObservable<V> {
    value: Mutex<V>,
    observers: Mutex<Vec<Observer<V>>>,
}

impl<V> Deref for AsyncObservable<V> {
    type Target = Mutex<V>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<V> AsyncObservable<V> {
    pub fn new(value: V) -> Self {
        Self {
            observers: Default::default(),
            value: Mutex::new(value),
        }
    }

    pub fn borrow(&self) -> MutexGuard<V> {
        self.value.lock().unwrap()
    }

    pub fn borrow_mut(&self) -> MutexGuard<V> {
        self.borrow()
    }

    /// add observer which is called when value under AsyncObservable changes
    pub fn add_observer(&self, observer: Observer<V>) {
        self.observers.lock().unwrap().push(observer);
    }

    /// notifies all observers that the value has changed
    pub fn notify(&self) {
        let mut observers = self.observers.lock().unwrap();
        let mut to_remove = vec![];

        {
            let mut x = 0usize;
            let value = self.value.lock().unwrap();
            for observer in &mut *observers {
                if (observer)(&value) {
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

impl<V> From<V> for AsyncObservable<V> {
    fn from(v: V) -> Self {
        Self::new(v)
    }
}
