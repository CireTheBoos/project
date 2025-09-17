mod create;
mod frame_renderer;
mod pipelines;
mod rendering_camera;
mod set_layouts;
mod swapchain;

use vk_mem::Allocator;

use crate::context::{Device, Instance, device::QueueRoleFlags};

use super::model::{self, Model};

use frame_renderer::FrameRenderer;
use pipelines::RenderingPipeline;
use rendering_camera::RenderingCamera;
use set_layouts::SetLayouts;
use swapchain::Swapchain;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

pub const FRAMES_IN_FLIGHT: usize = 2;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct Renderer {
    // swapchain
    swapchain: Swapchain,

    // frames
    next_frame_index: usize,
    frames: [FrameRenderer; FRAMES_IN_FLIGHT],

    // camera
    pub rendering_camera: RenderingCamera,

    // set layouts
    set_layouts: SetLayouts,

    // rendering pipeline
    rendering_pipeline: RenderingPipeline,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Create & Recreate & Destroy
impl Renderer {
    pub fn create(instance: &Instance, device: &Device, allocator: &Allocator) -> Result<Renderer> {
        create::create(instance, device, allocator)
    }

    pub fn recreate(
        &mut self,
        instance: &Instance,
        device: &Device,
        allocator: &Allocator,
    ) -> Result<()> {
        // wait end of work
        unsafe { device.queue_wait_idle(device.queue(QueueRoleFlags::RENDER).vk_queue) }?;
        unsafe { device.queue_wait_idle(device.queue(QueueRoleFlags::PRESENT).vk_queue) }?;

        // recreate swapchain
        self.swapchain.recreate(instance, device)?;

        // recreate frames
        for frame in self.frames.iter_mut() {
            frame.recreate(instance, device, allocator, self.swapchain.extent)?;
        }

        // update camera projection
        self.rendering_camera
            .update_projection(self.swapchain.extent);

        Ok(())
    }

    pub fn destroy(&mut self, device: &Device, allocator: &Allocator) {
        self.rendering_pipeline.destroy(device);
        self.set_layouts.destroy(device);
        self.rendering_camera.destroy(allocator);
        for frame in &mut self.frames {
            frame.destroy(device, allocator);
        }
        self.swapchain.destroy(device);
    }
}

/// Draw
impl Renderer {
    /// Success contains `true` if rendering need recreation (swapchain out of date or suboptimal).
    pub fn draw(&mut self, device: &Device, allocator: &Allocator, model: &Model) -> Result<bool> {
        // update before drawing
        self.rendering_camera.update_buffer(allocator)?;

        // draw with current frame
        let need_recreation = self.frames[self.next_frame_index].draw(
            device,
            &self.swapchain,
            &self.rendering_pipeline,
            model,
        )?;

        // step to next frame
        self.next_frame_index = (self.next_frame_index + 1) % FRAMES_IN_FLIGHT;

        Ok(need_recreation)
    }
}
