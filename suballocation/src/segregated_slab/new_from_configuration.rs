use mem_utils::RangeOf;
use rustc_hash::FxHashMap;

use super::{SegregatedSlabSuballocator, Slab};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////////
// Argument
/////////////////////////////////////////////////////////////////////////////

pub struct SegregatedSlabConfiguration<U> {
    range: RangeOf<U>,
    slab: usize,
    classes: Vec<usize>,
}

/// Constructors
impl<U> SegregatedSlabConfiguration<U> {
    /// Powers of Two (PoT):
    /// - Slab is `max_size`.
    /// - Classes are pot from `min_size to `max_size`.
    pub fn pot(
        range: RangeOf<U>,
        max_size: usize,
        min_size: usize,
    ) -> Result<SegregatedSlabConfiguration<U>> {
        // check pot
        if !range.size.is_power_of_two() {
            return Err("`range.size` should be power of two".into());
        }
        if !max_size.is_power_of_two() {
            return Err("`max_size` should be power of two".into());
        }
        if !min_size.is_power_of_two() {
            return Err("`min_size` should be power of two".into());
        }

        // check ordering
        if max_size > range.size {
            return Err("`max_size` > `range.size`".into());
        }
        if min_size > max_size {
            return Err("`min_size` > `max_size`".into());
        }

        // classes
        let mut classes = Vec::new();
        let mut pot = min_size;
        while pot <= max_size {
            classes.push(pot);
            pot *= 2;
        }

        Ok(SegregatedSlabConfiguration {
            range,
            slab: max_size,
            classes,
        })
    }
}

/////////////////////////////////////////////////////////////////////////////
// Fonction
/////////////////////////////////////////////////////////////////////////////

pub fn new_from_configuration<U>(
    configuration: SegregatedSlabConfiguration<U>,
) -> Result<SegregatedSlabSuballocator<U>> {
    let SegregatedSlabConfiguration {
        range,
        slab,
        classes,
    } = configuration;

    // slabs
    let slab_count = range.size / slab;
    let mut slabs = Vec::with_capacity(slab_count);
    for i in 0..slab_count {
        let slab_range = range.subrange(i * slab, slab).unwrap(); // UNWRAP: divisibility checked in configuration
        let slab = Slab::new(slab_range);
        slabs.push(slab);
    }

    // indices
    let empty_slab_indices = (0..slab_count).rev().collect();
    let mut partial_slab_indices_per_class = FxHashMap::default();
    for class in classes.iter().copied() {
        partial_slab_indices_per_class.insert(class, Vec::new());
    }

    Ok(SegregatedSlabSuballocator {
        range,
        slab,
        classes,
        slabs,
        empty_slab_indices,
        partial_slab_indices_per_class,
    })
}
