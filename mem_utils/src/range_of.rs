use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////////
/// Structure
/////////////////////////////////////////////////////////////////////////////

/// Range aware of its unit.
pub struct RangeOf<Unit = u8> {
    pub offset: usize,
    pub size: usize,
    pub(crate) unit: PhantomData<Unit>,
}

/////////////////////////////////////////////////////////////////////////////
/// Implementations
/////////////////////////////////////////////////////////////////////////////

/// New
impl<Unit> RangeOf<Unit> {
    pub fn new(offset: usize, size: usize) -> Self {
        Self {
            offset,
            size,
            unit: PhantomData,
        }
    }
}

/// Range-specific
impl<Unit> RangeOf<Unit> {
    pub fn to_std_range(&self) -> std::ops::Range<usize> {
        self.offset..self.end()
    }

    /// Fail if :
    /// - Sub range is not contained in `self`.
    pub fn subrange(&self, inner_offset: usize, size: usize) -> Result<Self> {
        if inner_offset + size <= self.size {
            Ok(Self::new(self.offset + inner_offset, size))
        } else {
            Err("sub range too big".into())
        }
    }

    /// offset + size => not included.
    pub fn end(&self) -> usize {
        self.offset + self.size
    }

    pub fn is_subrange_of(&self, range: &Self) -> bool {
        self.offset >= range.offset && self.end() <= range.end()
    }
}

/// Byte
impl<Unit> RangeOf<Unit> {
    pub fn byte_offset(&self) -> usize {
        self.offset * size_of::<Unit>()
    }

    pub fn byte_size(&self) -> usize {
        self.size * size_of::<Unit>()
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
impl<U> Display for RangeOf<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{};{}[ (size {})",
            self.offset,
            self.offset + self.size,
            self.size
        )
    }
}

/////////////////////////////////////////////////////////////////////////////
/// Trivial implementations : Debug, Clone, Copy, PartialEq, Eq
/////////////////////////////////////////////////////////////////////////////
/// Can't be derived because it would require U to implement them as well (even if phantom).

/// Debug
impl<U> Debug for RangeOf<U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RangeOf")
            .field("offset", &self.offset)
            .field("size", &self.size)
            .field("unit", &self.unit)
            .finish()
    }
}

/// Clone & Copy
impl<U> Clone for RangeOf<U> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<U> Copy for RangeOf<U> {}

/// PartialEq & Eq
impl<U> PartialEq for RangeOf<U> {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset && self.size == other.size
    }
}
impl<U> Eq for RangeOf<U> {}
