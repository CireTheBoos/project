use ash::vk;
use mem_utils::RangeOf;

use super::{ShapeInfo, Triangle, Vertex};

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct Shape {
    // cloud ranges
    pub cloud: RangeOf<Vertex>,
    pub visible_cloud: RangeOf<Vertex>,

    // surface ranges
    pub surface: RangeOf<Triangle>,
    pub visible_surface: RangeOf<Triangle>,

    // shape data
    pub diic: vk::DrawIndexedIndirectCommand,
    pub info: ShapeInfo,
}
