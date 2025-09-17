//! Segregated slab allocation algorithm.
//!
//! # Initialization
//!
//! - Define memory region from which we will allocate.
//!     - 32 bytes in example.
//!     - `#` := 1 byte.
//!
//! `################################`
//!
//! - Divide it in chunks of equal size called **slabs**.
//!     - 4 slabs of size 8 in example.
//!     - => Slab size must divide memory size.
//!
//! `######## ######## ######## ########`
//!
//! - Define possible item sizes that can be allocated, one per slab, called **classes**.
//!     - {2,4} in example.
//!     - => Each class must divide slab size.
//!     - `|x|` := class of the slab at its right.
//!
//! `|| ######## || ######## || ######## || ########`
//!
//! # Allocation
//!
//! ## Algorithm
//! 1. Find item's class (closest upper class).
//! 2. Search space among slabs of this class.
//! 3. If no space among them, then search a free slab to assign it item's class and allocate a slot from it.
//! 4. If no free slabs, then fail allocation.
//!
//! ## Examples
//!
//! - Allocate `aaa` (3 letters => 3 bytes).
//!     - `~` := padding bytes.
//!
//! `|4| aaa~#### || ######## || ######## || ########`
//!
//! - Allocate `bb`, `cccc`, `dddd` :
//!
//! `|4| aaa~#### |2| bb###### || ######## || ########`
//!
//! `|4| aaa~cccc |2| bb###### || ######## || ########`
//!
//! `|4| aaa~cccc |2| bb###### |4| dddd#### || ########`
//!
//! # Deallocation
//!
//! ## Examples
//!
//! Starting from last state at the end of "Allocation".
//!
//! - Free `aaa`, `dddd` :
//!
//! `|4| ####cccc |2| bb###### |4| dddd#### || ########`
//!
//! `|4| ####cccc |2| bb###### || ######## || ########`
//!
//! # Reallocation
//!
//! ## Examples
//!
//! Starting from last state at the end of "Deallocation".
//!
//! - Reallocate `cccc` to `c` (shrinking) :
//!
//! `|| ######## |2| bbc~#### || ######## || ########`
//!
//! - Reallocate `bb` to `b` (shrinking without changing class) :
//!
//! `|| ######## |2| b~c~#### || ######## || ########`
//!
//! - Reallocate `b` to `bbb` (growing) :
//!
//! `|4| bbb~#### |2| ##c~#### || ######## || ########`
//!
//! - Reallocate `bbb` to `bbbb` (growing without changing class) :
//!
//! `|4| bbbb#### |2| ##c~#### || ######## || ########`
//!
//! # Analysis
//!
//! ## Pros and cons
//!
//! Pros :
//! - No external fragmentation (entirety of each slab is possibly used).
//! - Fixed address/offset unless data is reallocated (which is explicit so under control).
//! - Minimal copying when reallocating, *if* it needs copying.
//!
//! Cons :
//! - Some internal fragmentation (wasted space when not using entirety of a class).
//! - More allocation failure when ratio "slabs / classes"  is low (below 2-3).
//! - No control on where data will be allocated (can't optimize for caching ? Although it might be negligeable if items are big enough).
//!
//! ## Tips
//!
//! Tips to choose slab size and classes :
//! - Badly chosen sizes can lead to more allocation failure than needed and internal fragmentation.
//! - Slab size = highest class is a good fit (maximize ratio "slabs / classes").
//! - Powers of 2 classes are great for varying item sizes (quite polyvalent, ~25% internal fragmentation).
//! - If item sizes are few and known in advance, use them for classes (0% internal fragmentation).
//!
//! # Todo
//!
//! - Might try overallocate by checking bigger class.
//! - Might add new logic to reallocation.

mod new_from_configuration;
mod slab;
#[cfg(test)]
mod test;

use mem_utils::RangeOf;
use rustc_hash::FxHashMap;

use super::ArrayOfUnitSuballocation;

pub use new_from_configuration::SegregatedSlabConfiguration;
use slab::{Occupation, Slab};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

/// Suballocate using segregated slab algorithm.
///
/// Invariants :
/// - slab index in `empty_slab_indices` <=> slab occupation == Occupation::Empty
/// - slab index in `partial_slab_indices_per_class` <=> slab occupation == Occupation::Partial
pub struct SegregatedSlabSuballocator<U> {
    range: RangeOf<U>, // immutable

    // sizes
    slab: usize,         // immutable
    classes: Vec<usize>, // immutable

    // slabs
    slabs: Vec<Slab<U>>,

    // indices
    empty_slab_indices: Vec<usize>,
    partial_slab_indices_per_class: FxHashMap<usize, Vec<usize>>,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// New
impl<U> SegregatedSlabSuballocator<U> {
    pub fn new_from_configuration(
        configuration: SegregatedSlabConfiguration<U>,
    ) -> Result<SegregatedSlabSuballocator<U>> {
        new_from_configuration::new_from_configuration::<U>(configuration)
    }
}

/// Utils
impl<U> SegregatedSlabSuballocator<U> {
    fn class_from(&self, size: usize) -> Result<usize> {
        if size == 0 {
            return Err("size null".into());
        }
        self.classes
            .iter()
            .find(|class| **class >= size)
            .copied()
            .ok_or("size too big".into())
    }

    fn slab_index_from(&self, range: RangeOf<U>) -> Result<usize> {
        if !range.is_subrange_of(&self.range) {
            return Err("out of bounds".into());
        }
        Ok((range.offset - self.range.offset) / self.slab)
    }
}

/// Suballocate
impl<U> ArrayOfUnitSuballocation<U> for SegregatedSlabSuballocator<U> {
    //------------// query //------------//

    fn can_allocate(&self, size: usize) -> bool {
        // check/extract `size` class
        let Ok(class) = self.class_from(size) else {
            return false;
        };

        // is there class partial slabs ?
        let partial_slab_indices = self.partial_slab_indices_per_class.get(&class).unwrap();
        let there_is_partial_slabs = !partial_slab_indices.is_empty();

        // is there empty slabs ?
        let there_is_empty_slabs = !self.empty_slab_indices.is_empty();

        //////
        there_is_partial_slabs || there_is_empty_slabs
    }

    fn is_allocated(&self, range: RangeOf<U>) -> bool {
        // check/extract `range` slab index
        let Ok(slab_index) = self.slab_index_from(range) else {
            return false;
        };

        //////
        self.slabs[slab_index].is_allocated(range)
    }

    fn can_reallocate(&self, range: RangeOf<U>, size: usize) -> bool {
        // is range allocated ?
        if !self.is_allocated(range) {
            return false;
        }

        // extract `range` slab class
        let slab_index = self.slab_index_from(range).unwrap(); // UNWRAP: is_allocated passed
        let slab_class = self.slabs[slab_index].class().unwrap(); // UNWRAP: is_allocated passed

        // is in place reallocation possible ?
        if size <= slab_class {
            return true;
        }

        // is a new allocation possible ?
        self.can_allocate(size)
    }

    //------------// allocate //------------//

    fn allocate(&mut self, size: usize) -> super::Result<RangeOf<U>> {
        if !self.can_allocate(size) {
            return Err("cannot allocate".into());
        }

        // extract `size` class partial slabs
        let class = self.class_from(size).unwrap(); // UNWRAP: can_allocate passed
        let partial_slab_indices = self.partial_slab_indices_per_class.get_mut(&class).unwrap();
        let there_is_partial_slabs = !partial_slab_indices.is_empty();

        if there_is_partial_slabs {
            // get partial slab
            let slab_index = partial_slab_indices.last().copied().unwrap();
            let slab = &mut self.slabs[slab_index];

            // allocate
            let allocated_range = unsafe { slab.allocate(size) };
            let new_occupation = slab.occupation();

            // update indices
            if new_occupation == Occupation::Full {
                partial_slab_indices.pop();
            }

            Ok(allocated_range)
        } else {
            // get empty slab
            let slab_index = self.empty_slab_indices.last().copied().unwrap(); // UNWRAP: can_allocate passed & no partial slabs
            let slab = &mut self.slabs[slab_index];

            // assign class
            slab.reset_slots(Some(class));

            // allocate
            let allocated_range = unsafe { slab.allocate(size) };
            let new_occupation = slab.occupation();

            // update indices
            self.empty_slab_indices.pop();
            if new_occupation == Occupation::Partial {
                // do nothing if `new_occupation` == `Occupation::Full`
                partial_slab_indices.push(slab_index);
            }

            Ok(allocated_range)
        }
    }

    fn deallocate(&mut self, range: RangeOf<U>) -> super::Result<()> {
        if !self.is_allocated(range) {
            return Err("not allocated".into());
        }

        // extract `range` slab & partial slabs
        let slab_index = self.slab_index_from(range).unwrap(); // UNWRAP: is_allocated passed
        let slab = &mut self.slabs[slab_index];
        let slab_class = slab.class().unwrap();
        let partial_slab_indices = self
            .partial_slab_indices_per_class
            .get_mut(&slab_class)
            .unwrap();

        // deallocate
        let old_occupation = slab.occupation();
        unsafe { slab.deallocate(range) };
        let new_occupation = slab.occupation();

        // update indices
        match (old_occupation, new_occupation) {
            (Occupation::Full, Occupation::Partial) => {
                // push to partial slabs
                partial_slab_indices.push(slab_index);
            }
            (Occupation::Full, Occupation::Empty) => {
                // push to empty slabs
                self.empty_slab_indices.push(slab_index);
            }
            (Occupation::Partial, Occupation::Partial) => {}
            (Occupation::Partial, Occupation::Empty) => {
                // remove from partial slabs
                let slab_index_position = partial_slab_indices
                    .iter()
                    .position(|index| *index == slab_index)
                    .unwrap(); // UNWRAP: old_occupation == Partial
                partial_slab_indices.swap_remove(slab_index_position);

                // push to empty slabs
                self.empty_slab_indices.push(slab_index);
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn reallocate(&mut self, range: RangeOf<U>, size: usize) -> super::Result<RangeOf<U>> {
        if !self.can_reallocate(range, size) {
            return Err("cannot reallocate".into());
        }

        // extract `range` slab class
        let slab_index = self.slab_index_from(range).unwrap(); // UNWRAP: can_reallocate passed
        let class = self.slabs[slab_index].class().unwrap(); // UNWRAP: can_reallocate passed

        // extract `size` class
        let new_class = self.class_from(size).unwrap(); // UNWRAP: can_reallocate passed

        // compare classes
        if new_class == class {
            // keep actual allocation
            unsafe { self.slabs[slab_index].reallocate_in_place(range, size) };
            Ok(RangeOf::new(range.offset, size))
        } else if new_class < class {
            // try reallocate smaller
            if let Ok(new_range) = self.allocate(size) {
                self.deallocate(range).unwrap(); // UNWRAP: can_reallocate passed
                Ok(new_range)
            } else {
                // keep actual allocation
                unsafe { self.slabs[slab_index].reallocate_in_place(range, size) };
                Ok(RangeOf::new(range.offset, size))
            }
        } else {
            // reallocate bigger or fail
            let new_range = self.allocate(size)?;
            self.deallocate(range).unwrap(); // UNWRAP: can_reallocate passed
            Ok(new_range)
        }
    }

    //------------// debug //------------//

    fn allocations(&self) -> Vec<RangeOf<U>> {
        let mut allocations = Vec::new();
        for slab in &self.slabs {
            allocations.append(&mut slab.allocations());
        }
        allocations
    }
}
