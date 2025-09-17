//! All clouds.
//!
//! `Cloud` :
//! - Size: Dynamic.
//! - Define shape's vertices (called its cloud) :
//!     - 1 position in shape space per vertex.
//!
//! # Full & visible clouds
//!
//! Clouds might have invisible vertices (where all its triangles interface another shape).
//!
//! To alleviate GPU load, only visible vertices are loaded.
//!
//! To do that we use 2 buffers :
//! - One on the CPU with the full clouds.
//! - One on the GPU with the "visible" clouds : Subclouds with only visible vertices.
//!
//! To ease copy operations, the visible part of the cloud is in the beginning.

mod full;
mod vertex;
mod visible;

use ash::vk;
use vk_mem::Allocator;

use crate::context::Device;

use full::FullClouds;
use visible::VisibleClouds;

pub use full::MAX_VERTICES_PER_FULL_CLOUD;
pub use vertex::Vertex;
pub use visible::MAX_VERTICES_PER_VISIBLE_CLOUD;

pub type VertexIndex = u16;
pub const INDEX_TYPE: vk::IndexType = vk::IndexType::UINT16;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct Clouds {
    pub full: FullClouds,
    pub visible: VisibleClouds,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Create & Destroy
impl Clouds {
    pub fn create(device: &Device, allocator: &Allocator) -> Result<Self> {
        Ok(Clouds {
            full: FullClouds::create(device, allocator)?,
            visible: VisibleClouds::create(device, allocator)?,
        })
    }

    pub fn destroy(&mut self, allocator: &Allocator) {
        self.full.destroy(allocator);
        self.visible.destroy(allocator);
    }
}
