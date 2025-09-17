//! All shapes.
//!
//! `Shape` :
//! - Size: Fixed.
//! - Reference shape's data :
//!     - Ranges : cloud + visible cloud + surface + visible surface.
//!     - Indices : material.
//!     - Uniques : poso.
//!
//! # Upload
//!
//! `Shape` contains *all* data.
//!
//! In the same way that cloud contains visible cloud that gets uploaded, shape contains the data to upload inside it.

mod diic;
mod full;
mod info;

use vk_mem::Allocator;

use crate::context::Device;

pub use full::Shape;
pub use info::ShapeInfo;

use diic::DiicShapes;
use full::FullShapes;
use info::InfoShapes;

use super::{Triangle, Vertex};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

pub const MAX_SHAPES: usize = 256;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct Shapes {
    pub full: FullShapes,
    pub diic: DiicShapes,
    pub info: InfoShapes,
}

/////////////////////////////////////////////////////////////////////////////
// Implemtations
/////////////////////////////////////////////////////////////////////////////

/// Create & Destroy
impl Shapes {
    pub fn create(device: &Device, allocator: &Allocator) -> Result<Self> {
        Ok(Self {
            full: FullShapes::create(device, allocator)?,
            diic: DiicShapes::create(device, allocator)?,
            info: InfoShapes::create(device, allocator)?,
        })
    }

    pub fn destroy(&mut self, allocator: &Allocator) {
        self.full.destroy(allocator);
        self.diic.destroy(allocator);
        self.info.destroy(allocator);
    }
}
