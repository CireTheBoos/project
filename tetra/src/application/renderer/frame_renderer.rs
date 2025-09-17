mod create;
mod depth;
mod draw;

use ash::vk;

use crate::context::{Device, Instance};

use super::{
    Swapchain,
    model::{self, Model},
    pipelines::{self, RenderingPipeline},
};

use depth::Depth;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct FrameRenderer {
    frame_index: usize,

    // commands
    command_pool: vk::CommandPool,
    render: vk::CommandBuffer,

    // sync
    swapchain_image_available_binary: vk::Semaphore,
    rendering_done_fence: vk::Fence,

    // resources
    pub depth: Depth,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Create & Destroy
impl FrameRenderer {
    pub fn create(
        instance: &Instance,
        device: &Device,
        allocator: &vk_mem::Allocator,
        frame_index: usize,
        swapchain_extent: vk::Extent2D,
    ) -> Result<Self> {
        create::create(instance, device, allocator, frame_index, swapchain_extent)
    }

    pub fn recreate(
        &mut self,
        instance: &Instance,
        device: &Device,
        allocator: &vk_mem::Allocator,
        new_swapchain_extent: vk::Extent2D,
    ) -> Result<()> {
        self.destroy(device, allocator);
        *self = Self::create(
            instance,
            device,
            allocator,
            self.frame_index,
            new_swapchain_extent,
        )?;
        Ok(())
    }

    pub fn destroy(&mut self, device: &Device, allocator: &vk_mem::Allocator) {
        unsafe {
            // commands
            device.destroy_command_pool(self.command_pool, None);

            // sync
            device.destroy_semaphore(self.swapchain_image_available_binary, None);
            device.destroy_fence(self.rendering_done_fence, None);

            // resources
            self.depth.destroy(device, allocator);
        }
    }
}

/// Draw
impl FrameRenderer {
    pub fn draw(
        &self,
        device: &Device,
        swapchain: &Swapchain,
        rendering_pipeline: &RenderingPipeline,
        model: &Model,
    ) -> Result<bool> {
        draw::draw(self, device, swapchain, rendering_pipeline, model)
    }
}
