mod camera;

use std::f32::consts::FRAC_PI_3;

use ash::vk;
use glam::{Mat4, Quat, Vec3};
use vk_mem::{Alloc, Allocator};

use camera::{Camera, Frustrum};

use crate::context::{Device, device::QueueRoleFlags};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

const FOV_Y_RADIANS: f32 = FRAC_PI_3;
const Z_NEAR: f32 = 0.1;
const Z_FAR: f32 = 100.;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct RenderingCamera {
    pub camera: Camera,
    pub allocation: vk_mem::Allocation,
    pub allocation_ptr: *mut u8,
    pub buffer: vk::Buffer,
}

impl RenderingCamera {
    pub const BUFFER_SIZE: u64 = size_of::<Mat4>() as u64 * 2;
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Create & Destroy
impl RenderingCamera {
    pub fn create(
        device: &Device,
        allocator: &Allocator,
        surface_extent: vk::Extent2D,
    ) -> Result<Self> {
        // camera
        let camera = Camera::new(
            Vec3::new(-1., 0., 3.),
            Quat::IDENTITY,
            Frustrum {
                fov_y_radians: FOV_Y_RADIANS,
                aspect_ratio: surface_extent.width as f32 / surface_extent.height as f32,
                z_near: Z_NEAR,
                z_far: Z_FAR,
            },
        );

        // buffer
        let queue_family_indices = device.queue_family_indices(QueueRoleFlags::RENDER);
        let buffer_info = vk::BufferCreateInfo::default()
            .size(Self::BUFFER_SIZE)
            .usage(vk::BufferUsageFlags::UNIFORM_BUFFER)
            .queue_family_indices(&queue_family_indices)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);
        let create_info = vk_mem::AllocationCreateInfo {
            flags: vk_mem::AllocationCreateFlags::MAPPED
                | vk_mem::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE,
            usage: vk_mem::MemoryUsage::Auto,
            ..Default::default()
        };
        let (buffer, mut allocation) =
            unsafe { allocator.create_buffer(&buffer_info, &create_info) }?;
        let allocation_ptr = unsafe { allocator.map_memory(&mut allocation) }?;

        Ok(Self {
            camera,
            allocation,
            allocation_ptr,
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

/// Update
impl RenderingCamera {
    pub fn update_projection(&mut self, new_surface_extent: vk::Extent2D) {
        self.camera.redo_projection(Frustrum {
            fov_y_radians: FOV_Y_RADIANS,
            aspect_ratio: new_surface_extent.width as f32 / new_surface_extent.height as f32,
            z_near: Z_NEAR,
            z_far: Z_FAR,
        });
    }

    pub fn update_buffer(&self, allocator: &Allocator) -> Result<()> {
        // write
        unsafe {
            (self.allocation_ptr as *mut Mat4).write(self.camera.view());
            (self.allocation_ptr as *mut Mat4)
                .add(1)
                .write(self.camera.projection);
        }

        // flush
        allocator.flush_allocation(&self.allocation, 0, vk::WHOLE_SIZE)?;

        Ok(())
    }
}
