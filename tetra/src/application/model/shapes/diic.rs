use ash::vk;
use vk_mem::{Alloc, Allocation, Allocator};

use crate::context::{Device, device::QueueRoleFlags};

use super::MAX_SHAPES;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct DiicShapes {
    pub allocation: Allocation,
    pub buffer: vk::Buffer,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Create & Destroy
impl DiicShapes {
    pub fn create(device: &Device, allocator: &Allocator) -> Result<Self> {
        // buffer info
        let queue_family_indices =
            device.queue_family_indices(QueueRoleFlags::TRANSFER | QueueRoleFlags::RENDER);
        let buffer_info = vk::BufferCreateInfo::default()
            .size((size_of::<vk::DrawIndexedIndirectCommand>() * MAX_SHAPES) as u64)
            .usage(
                vk::BufferUsageFlags::TRANSFER_DST
                    | vk::BufferUsageFlags::INDIRECT_BUFFER
                    | vk::BufferUsageFlags::UNIFORM_BUFFER,
            )
            .queue_family_indices(&queue_family_indices)
            .sharing_mode(vk::SharingMode::CONCURRENT);

        // allocation info
        let allocation_info = vk_mem::AllocationCreateInfo {
            usage: vk_mem::MemoryUsage::AutoPreferDevice,
            ..Default::default()
        };

        // create
        let (buffer, allocation) =
            unsafe { allocator.create_buffer(&buffer_info, &allocation_info) }?;

        Ok(Self { allocation, buffer })
    }

    pub fn destroy(&mut self, allocator: &Allocator) {
        unsafe {
            allocator.destroy_buffer(self.buffer, &mut self.allocation);
        }
    }
}
