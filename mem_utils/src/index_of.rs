use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

use super::RangeOf;

/////////////////////////////////////////////////////////////////////////////
/// Structure
/////////////////////////////////////////////////////////////////////////////

/// Index aware of its unit.
pub struct IndexOf<Unit = u8> {
    pub index: usize,
    unit: PhantomData<Unit>,
}

/////////////////////////////////////////////////////////////////////////////
/// Implementations
/////////////////////////////////////////////////////////////////////////////

/// New
impl<Unit> IndexOf<Unit> {
    pub fn new(index: usize) -> IndexOf<Unit> {
        IndexOf {
            index,
            unit: PhantomData,
        }
    }
}

/// Byte
impl<Unit> IndexOf<Unit> {
    pub fn byte_offset(&self) -> usize {
        self.index * size_of::<Unit>()
    }

    pub fn byte_size(&self) -> usize {
        size_of::<Unit>()
    }

    pub fn as_range_of_bytes(&self) -> RangeOf<u8> {
        RangeOf {
            offset: self.byte_offset(),
            size: self.byte_size(),
            unit: PhantomData,
        }
    }
}

/// Display
impl<U> Display for IndexOf<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.index,)
    }
}

/////////////////////////////////////////////////////////////////////////////
/// Trivial implementations : Debug, Clone, Copy, PartialEq, Eq
/////////////////////////////////////////////////////////////////////////////
/// Can't be derived because it would require U to implement them as well (even if phantom).

/// Debug
impl<U> Debug for IndexOf<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndexOf")
            .field("index", &self.index)
            .finish()
    }
}

/// Clone & Copy
impl<U> Clone for IndexOf<U> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<U> Copy for IndexOf<U> {}

/// PartialEq & Eq
impl<U> PartialEq for IndexOf<U> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}
impl<U> Eq for IndexOf<U> {}
