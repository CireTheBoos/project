use ash::vk;
use vk_mem::Allocator;

use crate::context::{Device, Instance};

use super::{FrameRenderer, Renderer, RenderingCamera, RenderingPipeline, SetLayouts, Swapchain};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////
// Function
/////////////////////////////////////////////////////////////////////////

pub fn create(instance: &Instance, device: &Device, allocator: &Allocator) -> Result<Renderer> {
    // swapchain
    let swapchain = Swapchain::create(instance, device)?;

    // frames
    let frames = [
        FrameRenderer::create(instance, device, allocator, 0, swapchain.extent)?,
        FrameRenderer::create(instance, device, allocator, 1, swapchain.extent)?,
    ];

    // camera
    let rendering_camera = RenderingCamera::create(device, allocator, swapchain.extent)?;

    // set layouts
    let set_layouts = SetLayouts::create(device)?;

    // rendering pipeline
    let rendering_pipeline = RenderingPipeline::create(
        device,
        &set_layouts.scene,
        swapchain.format,
        frames[0].depth.format,
    )?;

    // update
    set_layouts.scene.update_set(
        device,
        rendering_pipeline.scene_set,
        vk::DescriptorBufferInfo::default()
            .buffer(rendering_camera.buffer)
            .offset(0)
            .range(RenderingCamera::BUFFER_SIZE),
    );

    Ok(Renderer {
        swapchain,
        next_frame_index: 0,
        frames,
        rendering_camera,
        set_layouts,
        rendering_pipeline,
    })
}
