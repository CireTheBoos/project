mod add;
mod clouds;
mod create;
mod shapes;
mod surfaces;

use ash::vk;
use vk_mem::Allocator;

use crate::context::Device;

pub use add::ShapeData;
pub use clouds::{INDEX_TYPE, Vertex};
pub use shapes::{MAX_SHAPES, ShapeInfo};
pub use surfaces::Triangle;

use clouds::Clouds;
use shapes::{Shape, Shapes};
use surfaces::Surfaces;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct Model {
    // resources
    pub shapes: Shapes,
    pub clouds: Clouds,
    pub surfaces: Surfaces,

    // commands
    transfer_command_pool: vk::CommandPool,
    add: vk::CommandBuffer,

    // sync
    ready_to_add: vk::Fence,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Create & Destroy
impl Model {
    pub fn create(device: &Device, allocator: &Allocator) -> Result<Self> {
        create::create(device, allocator)
    }

    pub fn destroy(&mut self, device: &Device, allocator: &Allocator) {
        // resources
        self.surfaces.destroy(allocator);
        self.clouds.destroy(allocator);
        self.shapes.destroy(allocator);

        // commands
        unsafe { device.destroy_command_pool(self.transfer_command_pool, None) };

        // sync
        unsafe { device.destroy_fence(self.ready_to_add, None) };
    }
}

/// Add
impl Model {
    pub fn add(&mut self, device: &Device, allocator: &Allocator, shape: ShapeData) -> Result<()> {
        add::add(self, device, allocator, shape)
    }
}
