use std::any::Any;
use std::ops::Range;

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

    pub fn compute_index_helper(
        index_buff: &mut [usize],
        dynamic_index: usize,
        dynamic_places_occupied: usize,
        constant_places_previously_occupied: usize,
    ) -> (usize, Range<usize>, bool) {
        let index: usize = index_buff[..dynamic_index].iter().sum::<usize>()
            + constant_places_previously_occupied
            - 1;

        let should_replace = index_buff[dynamic_index] != 0;

        let range_to_remove = if dynamic_places_occupied >= index_buff[dynamic_index] {
            0..0
        } else {
            let start = index + dynamic_places_occupied;
            start..(start + index_buff[dynamic_index])
        };

        index_buff[dynamic_index] = dynamic_places_occupied;

        (index, range_to_remove, should_replace)
    }
}
