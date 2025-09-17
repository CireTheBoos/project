//! Unit allocator using an empty slot table.

use mem_utils::{IndexOf, RangeOf};

use super::UnitSuballocation;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct TableSuballocator<U> {
    range: RangeOf<U>,
    slots: Vec<Slot<U>>,

    // indices
    empty_slot_indices: Vec<usize>,
}

struct Slot<U> {
    index: IndexOf<U>,
    is_allocated: bool,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// New
impl<U> TableSuballocator<U> {
    pub fn new(range: RangeOf<U>) -> TableSuballocator<U> {
        let slots = (range.offset..range.offset + range.size)
            .map(|index| Slot {
                index: IndexOf::new(index),
                is_allocated: false,
            })
            .collect();
        let empty_slot_indices = (0..range.size).rev().collect();
        TableSuballocator {
            range,
            slots,
            empty_slot_indices,
        }
    }
}

/// Utils
impl<U> TableSuballocator<U> {
    fn slot_index_from(&self, index: IndexOf<U>) -> Result<usize> {
        if !self.range.to_std_range().contains(&index.index) {
            return Err("out of bounds".into());
        }
        Ok(index.index - self.range.offset)
    }
}

/// Suballocate
impl<U> UnitSuballocation<U> for TableSuballocator<U> {
    //------// query //------//

    fn can_allocate(&self) -> bool {
        !self.empty_slot_indices.is_empty()
    }

    fn is_allocated(&self, index: IndexOf<U>) -> bool {
        let Ok(slot_index) = self.slot_index_from(index) else {
            return false;
        };
        self.slots[slot_index].is_allocated
    }

    //------// suballocate //------//

    fn allocate(&mut self) -> crate::Result<IndexOf<U>> {
        // check
        if !self.can_allocate() {
            return Err("cannot allocate".into());
        }

        // extract empty slot
        let slot_index = self.empty_slot_indices.pop().unwrap(); // UNWRAP: can_allocate passed
        let slot = &mut self.slots[slot_index];

        // allocate
        slot.is_allocated = true;
        Ok(slot.index)
    }

    fn deallocate(&mut self, index: IndexOf<U>) -> crate::Result<()> {
        // check
        if !self.is_allocated(index) {
            return Err("cannot deallocate".into());
        }

        // extract `index` slot
        let slot_index = self.slot_index_from(index).unwrap(); // UNWRAP: is_allocated passed
        let slot = &mut self.slots[slot_index];

        // deallocate
        slot.is_allocated = false;
        Ok(())
    }

    //------// debug //------//

    fn allocations(&self) -> Vec<IndexOf<U>> {
        self.slots
            .iter()
            .filter_map(|slot| slot.is_allocated.then_some(slot.index))
            .collect()
    }
}
