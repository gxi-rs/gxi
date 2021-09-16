use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct State<T: Default> {
    state: T,
    is_dirty: bool,
}

impl<T: Default> Deref for State<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<T: Default> DerefMut for State<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.mark_dirty();
        &mut self.state
    }
}

impl<T: Default> State<T> {
    pub fn is_dirty(&self) -> bool {
        self.is_dirty.clone()
    }
    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }
    pub fn mark_clean(&mut self) {
        self.is_dirty = false;
    }
}
