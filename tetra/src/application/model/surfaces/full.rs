use ash::vk;
use mem_utils::RangeOf;
use suballocation::segregated_slab::{SegregatedSlabConfiguration, SegregatedSlabSuballocator};
use vk_mem::{Alloc, Allocation, Allocator};

use crate::context::{Device, device::QueueRoleFlags};

use super::Triangle;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

pub const MAX_TRIANGLES_PER_FULL_SURFACE: usize = 256;

const MIN_TRIANGLES_PER_FULL_SURFACE: usize = 4; // at least one tetra so 4 triangles
const MAX_TRIANGLES: usize = 4096;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct FullSurfaces {
    pub allocation: Allocation,
    pub buffer: vk::Buffer,
    pub memory: &'static mut [Triangle],
    pub suballocator: SegregatedSlabSuballocator<Triangle>,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Create & Destroy
impl FullSurfaces {
    pub fn create(device: &Device, allocator: &Allocator) -> Result<Self> {
        // buffer
        let queue_family_indices = device.queue_family_indices(QueueRoleFlags::TRANSFER);
        let buffer_info = vk::BufferCreateInfo::default()
            .size((size_of::<Triangle>() * MAX_TRIANGLES) as u64)
            .usage(vk::BufferUsageFlags::TRANSFER_SRC)
            .queue_family_indices(&queue_family_indices)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        // allocation
        let allocation_info = vk_mem::AllocationCreateInfo {
            flags: vk_mem::AllocationCreateFlags::MAPPED
                | vk_mem::AllocationCreateFlags::HOST_ACCESS_RANDOM,
            usage: vk_mem::MemoryUsage::AutoPreferHost,
            ..Default::default()
        };

        // create
        let (buffer, mut allocation) =
            unsafe { allocator.create_buffer(&buffer_info, &allocation_info) }?;
        let allocation_ptr = unsafe { allocator.map_memory(&mut allocation) }? as *mut Triangle;

        // memory
        let memory = unsafe { std::slice::from_raw_parts_mut(allocation_ptr, MAX_TRIANGLES) };

        // suballocator
        let suballocator =
            SegregatedSlabSuballocator::new_from_configuration(SegregatedSlabConfiguration::pot(
                RangeOf::new(0, MAX_TRIANGLES),
                MAX_TRIANGLES_PER_FULL_SURFACE,
                MIN_TRIANGLES_PER_FULL_SURFACE,
            )?)?;

        Ok(Self {
            allocation,
            memory,
            suballocator,
            buffer,
        })
    }

    pub fn destroy(&mut self, allocator: &Allocator) {
        unsafe {
            allocator.unmap_memory(&mut self.allocation);
            allocator.destroy_buffer(self.buffer, &mut self.allocation);
        }
    }
}
