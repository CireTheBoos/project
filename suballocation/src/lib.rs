//! # Allocating VS Suballocating
//!
//! Allocating means request memory from the OS :
//! - It's *costly* (idk why, all system calls are costly maybe).
//! - *fragmentation* isn't controlled (OS memory is shared between programs).
//!
//! Suballocating means request memory from *already owned memory*.
//! - No cost.
//! - Fragmentation is controlled.
//!
//! Vulkan recommnend suballocating for performance.
//!
//! # Levels
//!
//! When managing dynamic data, I suballocate twice.
//! 1. First VMA allocate a memory chunk of 256Mb and suballocate from it to back up a requested buffer (as vulkan recommend).
//! 2. Second, I suballocate *from the buffer memory* to manage dynamically-sized objects.

pub mod segregated_slab;
pub mod table;

use mem_utils::{IndexOf, RangeOf};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////////
// Traits
/////////////////////////////////////////////////////////////////////////////

pub trait ArrayOfUnitSuballocation<Unit> {
    //------------// query //------------//
    fn can_allocate(&self, size: usize) -> bool;
    fn is_allocated(&self, range: RangeOf<Unit>) -> bool;
    fn can_reallocate(&self, range: RangeOf<Unit>, size: usize) -> bool;

    //------------// suballocate //------------//
    fn allocate(&mut self, size: usize) -> Result<RangeOf<Unit>>;
    fn deallocate(&mut self, range: RangeOf<Unit>) -> Result<()>;
    fn reallocate(&mut self, range: RangeOf<Unit>, size: usize) -> Result<RangeOf<Unit>>;

    //------------// debug //------------//
    fn allocations(&self) -> Vec<RangeOf<Unit>>;
}

pub trait UnitSuballocation<Unit> {
    //------------// query //------------//
    fn can_allocate(&self) -> bool;
    fn is_allocated(&self, index: IndexOf<Unit>) -> bool;

    //------------// suballocate //------------//
    fn allocate(&mut self) -> Result<IndexOf<Unit>>;
    fn deallocate(&mut self, index: IndexOf<Unit>) -> Result<()>;

    //------------// debug //------------//
    fn allocations(&self) -> Vec<IndexOf<Unit>>;
}
