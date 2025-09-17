use std::fmt::Display;

use mem_utils::RangeOf;

/////////////////////////////////////////////////////////////////////////////
// Structures
/////////////////////////////////////////////////////////////////////////////

pub struct Slab<U> {
    range: RangeOf<U>,           // immutable
    slots: Option<Vec<Slot<U>>>, // `None` <=> slab free
}

struct Slot<U> {
    range: RangeOf<U>,           // immutable
    is_allocated: Option<usize>, // `Some(allocated_size)` & `None` <=> slot free
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// New & Reset
impl<U> Slab<U> {
    pub fn new(range: RangeOf<U>) -> Slab<U> {
        Slab { range, slots: None }
    }

    pub fn reset_slots(&mut self, class: Option<usize>) {
        // map `Some(class)` to `Some(slots)`
        self.slots = class.map(|class| {
            // index to slot
            let index_to_slot = |i| Slot {
                range: RangeOf::new(self.range.offset + i * class, class),
                is_allocated: None,
            };

            // create slots from indices
            let slot_count = self.range.size / class;
            (0..slot_count).map(index_to_slot).collect()
        });
    }
}

/// Occupation
impl<U> Slab<U> {
    pub fn occupation(&self) -> Occupation {
        if let Some(ref slots) = self.slots {
            // count free slots
            let mut free_slots = 0;
            for slot in slots {
                if slot.is_allocated.is_none() {
                    free_slots += 1;
                }
            }

            // match to occupation
            match free_slots {
                0 => Occupation::Full,
                n if n == slots.len() => Occupation::Empty,
                _ => Occupation::Partial,
            }
        } else {
            Occupation::Empty
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Occupation {
    Empty,
    Partial,
    Full,
}

/// Query
impl<U> Slab<U> {
    #[allow(clippy::manual_map)]
    pub fn class(&self) -> Option<usize> {
        if let Some(ref slots) = self.slots {
            Some(slots[0].range.size)
        } else {
            None
        }
    }

    pub fn is_allocated(&self, range: RangeOf<U>) -> bool {
        if let Some(ref slots) = self.slots {
            slots.iter().any(|slot| {
                slot.range.offset == range.offset && slot.is_allocated == Some(range.size)
            })
        } else {
            false // not allocated if slab is free
        }
    }

    pub fn allocations(&self) -> Vec<RangeOf<U>> {
        let mut allocations = Vec::new();

        // empty if slab is free
        if let Some(ref slots) = self.slots {
            for slot in slots {
                if let Some(allocated_size) = slot.is_allocated {
                    allocations.push(RangeOf::new(slot.range.offset, allocated_size));
                }
            }
        }

        allocations
    }
}

/// Display
impl<U> Display for Slab<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref slots) = self.slots {
            // extract
            let class = self.class().unwrap();
            let free_slot_count = slots
                .iter()
                .filter(|slot| slot.is_allocated.is_none())
                .count();
            let slot_count = self.range.size / class;

            // display
            write!(
                f,
                "slab with range : {}, class : {}, free slots remaining : {} out of {}",
                self.range, class, free_slot_count, slot_count,
            )
        } else {
            write!(f, "free slab with range : {}", self.range)
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
// Unsafe implementations
/////////////////////////////////////////////////////////////////////////////

/// Allocate & Deallocate & Reallocate
impl<U> Slab<U> {
    /// Unsafe if :
    /// - `self` free.
    /// - `self` full.
    /// - `size` bigger than self class.
    pub unsafe fn allocate(&mut self, size: usize) -> RangeOf<U> {
        // find
        let slot = unsafe {
            self.slots
                .as_mut()
                .unwrap_unchecked()
                .iter_mut()
                .find(|slot| slot.is_allocated.is_none())
                .unwrap_unchecked()
        };

        // allocate
        slot.is_allocated = Some(size);

        RangeOf::new(slot.range.offset, size)
    }

    /// Unsafe if :
    /// - `self` free.
    /// - `range` not allocated.
    pub unsafe fn deallocate(&mut self, range: RangeOf<U>) {
        // find
        let slot = unsafe {
            self.slots
                .as_mut()
                .unwrap_unchecked()
                .iter_mut()
                .find(|slot| slot.range.offset == range.offset)
                .unwrap_unchecked()
        };

        // deallocate
        slot.is_allocated = None;
    }

    /// Unsafe if :
    /// - `self` free.
    /// - `range` not allocated.
    /// - `size` bigger than self class.
    pub unsafe fn reallocate_in_place(&mut self, range: RangeOf<U>, size: usize) -> RangeOf<U> {
        // find
        let slot = unsafe {
            self.slots
                .as_mut()
                .unwrap_unchecked()
                .iter_mut()
                .find(|slot| slot.range.offset == range.offset)
                .unwrap_unchecked()
        };

        // reallocate
        slot.is_allocated = Some(size);

        RangeOf::new(slot.range.offset, size)
    }
}
