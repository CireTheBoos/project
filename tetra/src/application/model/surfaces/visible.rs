use ash::vk;
use mem_utils::RangeOf;
use suballocation::segregated_slab::{SegregatedSlabConfiguration, SegregatedSlabSuballocator};
use vk_mem::{Alloc, Allocation, Allocator};

use crate::context::{Device, device::QueueRoleFlags};

use super::Triangle;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

pub const MAX_TRIANGLES_PER_VISIBLE_SURFACE: usize = 256;

const MIN_TRIANGLES_PER_VISIBLE_SURFACE: usize = 1; // at least 1 visible triangle
const MAX_TRIANGLES: usize = 4096;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct VisibleSurfaces {
    pub allocation: Allocation,
    pub buffer: vk::Buffer,
    pub suballocator: SegregatedSlabSuballocator<Triangle>,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Create & Destroy
impl VisibleSurfaces {
    pub fn create(device: &Device, allocator: &Allocator) -> Result<Self> {
        // buffer info
        let queue_family_indices =
            device.queue_family_indices(QueueRoleFlags::TRANSFER | QueueRoleFlags::RENDER);
        let buffer_info = vk::BufferCreateInfo::default()
            .size((size_of::<Triangle>() * MAX_TRIANGLES) as u64)
            .usage(vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::INDEX_BUFFER)
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

        // suballocator
        let suballocator =
            SegregatedSlabSuballocator::new_from_configuration(SegregatedSlabConfiguration::pot(
                RangeOf::new(0, MAX_TRIANGLES),
                MAX_TRIANGLES_PER_VISIBLE_SURFACE,
                MIN_TRIANGLES_PER_VISIBLE_SURFACE,
            )?)?;

        Ok(Self {
            allocation,
            suballocator,
            buffer,
        })
    }

    pub fn destroy(&mut self, allocator: &Allocator) {
        unsafe {
            allocator.destroy_buffer(self.buffer, &mut self.allocation);
        }
    }
}
