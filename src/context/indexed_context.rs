use std::ops::Range;

use crate::MemDump;

#[derive(Default, derive_more::Deref, derive_more::DerefMut)]
pub struct IndexedContext {
    pub index: usize,
    #[deref]
    #[deref_mut]
    pub ctx: MemDump,
}

impl IndexedContext {
    /// if current index is not equal to provided index
    /// then mutates current index and returns true
    /// drops indexed value
    pub fn check_index(&mut self, index: usize) -> bool {
        if self.index != index {
            self.ctx.clear();
            self.index = index;
            true
        } else {
            false
        }
    }

    /// sets *self to default
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    pub fn compute_index(
        index_buff: &mut [usize],
        index_buff_index: usize,
        dynamic_places_occupied: usize,
        constant_places_occupied: usize,
    ) -> (usize, Range<usize>, bool) {
        let index: usize =
            index_buff[..index_buff_index].iter().sum::<usize>() + constant_places_occupied - 1;

        let should_replace = index_buff[index_buff_index] != 0;

        let range_to_remove = if dynamic_places_occupied >= index_buff[index_buff_index] {
            0..0
        } else {
            let start = index + dynamic_places_occupied;
            start..(start + index_buff[index_buff_index])
        };

        index_buff[index_buff_index] = dynamic_places_occupied;

        (index, range_to_remove, should_replace)
    }
}
