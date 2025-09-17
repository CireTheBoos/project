//! All surfaces.
//!
//! `Surface` :
//! - Size: Dynamic.
//! - Define shape's external triangles (called its surface) :
//!     - 3 shape's cloud index per triangle.
//!
//! # Full & visible surfaces
//!
//! Surfaces might have invisible triangles that interface another shape.
//!
//! To alleviate GPU load, only visible triangles are loaded.
//!
//! To do that we use 2 buffers :
//! - One on the CPU with the full surfaces.
//! - One on the GPU with the "visible" surfaces : Subsurfaces with only visible triangles.
//!
//! To ease copy operations, the visible part of the surface is in the beginning.

mod full;
mod triangle;
mod visible;

use vk_mem::Allocator;

use crate::context::Device;

use super::clouds::VertexIndex;

use full::FullSurfaces;
use visible::VisibleSurfaces;

pub use full::MAX_TRIANGLES_PER_FULL_SURFACE;
pub use triangle::Triangle;
pub use visible::MAX_TRIANGLES_PER_VISIBLE_SURFACE;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct Surfaces {
    pub full: FullSurfaces,
    pub visible: VisibleSurfaces,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Create & Destroy
impl Surfaces {
    pub fn create(device: &Device, allocator: &Allocator) -> Result<Self> {
        Ok(Self {
            full: FullSurfaces::create(device, allocator)?,
            visible: VisibleSurfaces::create(device, allocator)?,
        })
    }

    pub fn destroy(&mut self, allocator: &Allocator) {
        self.full.destroy(allocator);
        self.visible.destroy(allocator);
    }
}
