use std::ptr;

use super::RangeOf;

/////////////////////////////////////////////////////////////////////////////

/// Copy size is necessary minimal.
pub fn copy_within_memory_nonoverlapping<U>(memory: &mut [U], src_range: RangeOf<U>, dst_range: RangeOf<U>) {
    let copy_size = usize::min(src_range.size, dst_range.size);
    unsafe {
        let src = memory.as_ptr().add(src_range.offset);
        let dst = memory.as_mut_ptr().add(dst_range.offset);
        ptr::copy_nonoverlapping(src, dst, copy_size);
    }
}