use std::ops::{Deref, DerefMut};

/// bool: true -> remove observer
pub type Observer<V> = Box<dyn FnMut(&V) -> bool>;

pub struct Observers<V>(Vec<Observer<V>>);

impl<V> Default for Observers<V> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<V> Deref for Observers<V> {
    type Target = Vec<Observer<V>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<V> DerefMut for Observers<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<V> Observers<V> {
    /// add observer which is called when value under Observers changes
    pub fn add_observer(&mut self, observer: Observer<V>) {
        self.push(observer);
    }

    /// notifies all observers that the value has changed
    pub fn notify(&mut self, value: &V) {
        let mut to_remove = vec![];

        {
            let mut x = 0usize;
            for observer in self.iter_mut() {
                if (observer)(value) {
                    to_remove.push(x);
                } else {
                    x += 1;
                }
            }
        }

        for x in to_remove {
            drop(self.remove(x));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Observers;
    #[test]
    fn observer_remove() {
        let obs = Observers::default();
        obs.add_observer(Box::new(|_| true));
        obs.add_observer(Box::new(|_| false));
        obs.add_observer(Box::new(|_| true));
        obs.notify(&());
        assert_eq!(obs.len(), 1);
    }
}
