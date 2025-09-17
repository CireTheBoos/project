use ash::vk;
use mem_utils::RangeOf;
use suballocation::segregated_slab::{SegregatedSlabConfiguration, SegregatedSlabSuballocator};
use vk_mem::{Alloc, Allocation, Allocator};

use crate::context::{Device, device::QueueRoleFlags};

use super::Vertex;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

pub const MAX_VERTICES_PER_VISIBLE_CLOUD: usize = 256; // <= VertexIndex::MAX

const MIN_VERTICES_PER_VISIBLE_CLOUD: usize = 4; // at least one triangle so 3 vertices, next pot = 4
const MAX_VERTICES: usize = 4096;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct VisibleClouds {
    pub allocation: Allocation,
    pub buffer: vk::Buffer,
    pub suballocator: SegregatedSlabSuballocator<Vertex>,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Create & Destroy
impl VisibleClouds {
    pub fn create(device: &Device, allocator: &Allocator) -> Result<Self> {
        // buffer info
        let queue_family_indices =
            device.queue_family_indices(QueueRoleFlags::TRANSFER | QueueRoleFlags::RENDER);
        let buffer_info = vk::BufferCreateInfo::default()
            .size((size_of::<Vertex>() * MAX_VERTICES) as u64)
            .usage(vk::BufferUsageFlags::TRANSFER_DST | vk::BufferUsageFlags::VERTEX_BUFFER)
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
                RangeOf::new(0, MAX_VERTICES),
                MAX_VERTICES_PER_VISIBLE_CLOUD,
                MIN_VERTICES_PER_VISIBLE_CLOUD,
            )?)?;

        Ok(Self {
            allocation,
            buffer,
            suballocator,
        })
    }

    pub fn destroy(&mut self, allocator: &Allocator) {
        unsafe {
            allocator.destroy_buffer(self.buffer, &mut self.allocation);
        }
    }
}
