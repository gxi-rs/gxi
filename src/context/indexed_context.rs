use std::any::Any;

pub struct IndexedContextNode {
    pub index: usize,
    pub value: Box<dyn Any>,
}

impl Default for IndexedContextNode {
    fn default() -> Self {
        Self {
            index: Default::default(),
            value: Box::from(()),
        }
    }
}

impl IndexedContextNode {
    /// if current index is not equal to provided index
    /// then mutates current index and returns true
    /// drops indexed value
    pub fn check_index(&mut self, index: usize) -> bool {
        if self.index != index {
            self.value = Box::from(());
            self.index = index;
            true
        } else {
            false
        }
    }

    pub fn set_value(&mut self, value: Box<dyn Any>) {
        self.value = value;
    }

    /// sets index to 0
    /// sets value to ()
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
