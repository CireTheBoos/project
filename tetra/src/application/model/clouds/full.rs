use ash::vk;
use mem_utils::RangeOf;
use suballocation::segregated_slab::{SegregatedSlabConfiguration, SegregatedSlabSuballocator};
use vk_mem::{Alloc, Allocation, Allocator};

use crate::context::{Device, device::QueueRoleFlags};

use super::Vertex;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

pub const MAX_VERTICES_PER_FULL_CLOUD: usize = 256; // <= VertexIndex::MAX

const MIN_VERTICES_PER_FULL_CLOUD: usize = 4; // at least one tetra so 4 vertices
const MAX_VERTICES: usize = 4096;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct FullClouds {
    pub allocation: Allocation,
    pub buffer: vk::Buffer,
    pub memory: &'static mut [Vertex],
    pub suballocator: SegregatedSlabSuballocator<Vertex>,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Create & Destroy
impl FullClouds {
    pub fn create(device: &Device, allocator: &Allocator) -> Result<Self> {
        // buffer
        let queue_family_indices = device.queue_family_indices(QueueRoleFlags::TRANSFER);
        let buffer_info = vk::BufferCreateInfo::default()
            .size((size_of::<Vertex>() * MAX_VERTICES) as u64)
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

        // memory
        let allocation_ptr = unsafe { allocator.map_memory(&mut allocation) }? as *mut Vertex;
        let memory = unsafe { std::slice::from_raw_parts_mut(allocation_ptr, MAX_VERTICES) };

        // suballocator
        let suballocator =
            SegregatedSlabSuballocator::new_from_configuration(SegregatedSlabConfiguration::pot(
                RangeOf::new(0, MAX_VERTICES),
                MAX_VERTICES_PER_FULL_CLOUD,
                MIN_VERTICES_PER_FULL_CLOUD,
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
