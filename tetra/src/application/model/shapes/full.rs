mod shape;

use ash::vk;
use mem_utils::RangeOf;
use suballocation::table::TableSuballocator;
use vk_mem::{Alloc, Allocation, Allocator};

use crate::context::{Device, device::QueueRoleFlags};

use super::{MAX_SHAPES, ShapeInfo, Triangle, Vertex};

pub use shape::Shape;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct FullShapes {
    pub allocation: Allocation,
    pub memory: &'static mut [Shape],
    pub suballocator: TableSuballocator<Shape>,
    pub buffer: vk::Buffer,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Create & Destroy
impl FullShapes {
    pub fn create(device: &Device, allocator: &Allocator) -> Result<Self> {
        // buffer
        let queue_family_indices = device.queue_family_indices(QueueRoleFlags::TRANSFER);
        let buffer_info = vk::BufferCreateInfo::default()
            .size((size_of::<Shape>() * MAX_SHAPES) as u64)
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
        let allocation_ptr = unsafe { allocator.map_memory(&mut allocation) }? as *mut Shape;

        // memory
        let memory = unsafe { std::slice::from_raw_parts_mut(allocation_ptr, MAX_SHAPES) };

        // suballocator
        let suballocator = TableSuballocator::new(RangeOf::new(0, MAX_SHAPES));

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
