mod record_render;

use ash::vk;

use super::{
    Depth, FrameRenderer, Swapchain,
    model::{self, Model},
    pipelines::{self, RenderingPipeline},
};
use crate::context::{Device, device::QueueRoleFlags};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////////
// Function
/////////////////////////////////////////////////////////////////////////////

/// Return `recreate_swapchain` bool.
///
/// Block if :
/// - No swapchain image available.
/// - Last rendering not finished.
pub fn draw(
    frame_renderer: &FrameRenderer,
    device: &Device,
    swapchain: &Swapchain,
    rendering_pipeline: &RenderingPipeline,
    model: &Model,
) -> Result<bool> {
    // extract (per frame)
    let rendering_done_fence = frame_renderer.rendering_done_fence;
    let swapchain_image_available_binary = frame_renderer.swapchain_image_available_binary;
    let command_pool = frame_renderer.command_pool;
    let render = frame_renderer.render;

    //------// block //------//

    // wait last rendering done
    let fences = [rendering_done_fence];
    unsafe {
        device.wait_for_fences(&fences, false, u64::MAX)?;
    }

    // acquire swapchain image
    let acquire_result = unsafe {
        device.swapchain_device.acquire_next_image(
            **swapchain,
            u64::MAX,
            swapchain_image_available_binary,
            vk::Fence::null(),
        )
    };

    // check for recreation
    match acquire_result {
        // recreate if :  Ok but suboptimal | Err out of date
        Ok((_, true)) | Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
            return Ok(true);
        }
        _ => (),
    }

    // extract (per image)
    let swapchain_image_index = acquire_result?.0;
    let swapchain_image_presentable_binary =
        swapchain.image_presentable_binaries[swapchain_image_index as usize];

    // reset rendering done fence when we're sure to submit rendering work
    let fences = [rendering_done_fence];
    unsafe {
        device.reset_fences(&fences)?;
    }

    //------// record //------//

    // reset command buffers
    unsafe { device.reset_command_pool(command_pool, vk::CommandPoolResetFlags::empty()) }?;

    // render
    record_render::record_render(
        device,
        model,
        render,
        swapchain,
        swapchain_image_index,
        &frame_renderer.depth,
        rendering_pipeline,
    )?;

    //------// submit //------//

    // command buffers
    let command_buffer_info = vk::CommandBufferSubmitInfo::default().command_buffer(render);
    let command_buffer_infos = [command_buffer_info];

    // wait semaphores
    let wait_semaphore_info = vk::SemaphoreSubmitInfo::default()
        .semaphore(swapchain_image_available_binary)
        .stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT);
    let wait_semaphore_infos = [wait_semaphore_info];

    // signal semaphores
    let signal_semaphore_info = vk::SemaphoreSubmitInfo::default()
        .semaphore(swapchain_image_presentable_binary)
        .stage_mask(vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT);
    let signal_semaphore_infos = [signal_semaphore_info];

    // submit
    let submit = vk::SubmitInfo2::default()
        .command_buffer_infos(&command_buffer_infos)
        .wait_semaphore_infos(&wait_semaphore_infos)
        .signal_semaphore_infos(&signal_semaphore_infos);

    // submits
    let submits = [submit];

    // queue submit
    unsafe {
        device.queue_submit2(
            device.queue(QueueRoleFlags::RENDER).vk_queue,
            &submits,
            rendering_done_fence,
        )
    }?;

    //------// present //------//

    // present
    let wait_binary_semaphores = [swapchain_image_presentable_binary];
    let present_result = device.queue_present_single_swapchain(
        **swapchain,
        swapchain_image_index,
        &wait_binary_semaphores,
    );

    // check for recreation
    match present_result {
        // recreate if :  Ok but suboptimal | Err out of date
        Ok(true) | Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
            return Ok(true);
        }
        _ => (),
    }

    Ok(false)
}
