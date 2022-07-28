use std::any::Any;

#[derive(Default)]
pub struct IndexedContext {
    pub index: usize,
    pub value: Option<Box<dyn Any>>,
}

impl IndexedContext {
    /// if current index is not equal to provided index
    /// then mutates current index and returns true
    /// drops indexed value
    pub fn check_index(&mut self, index: usize) -> bool {
        if self.index != index {
            self.value = None;
            self.index = index;
            true
        } else {
            false
        }
    }

    pub fn set_value(&mut self, value: Box<dyn Any>) {
        self.value = Some(value);
    }

    /// sets *self to default
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
