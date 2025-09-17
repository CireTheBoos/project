use std::ffi::CString;

use ash::vk;

use super::{Depth, FrameRenderer};
use crate::context::{Device, Instance, device::QueueRoleFlags};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////
// Function
/////////////////////////////////////////////////////////////////////////

pub fn create(
    instance: &Instance,
    device: &Device,
    allocator: &vk_mem::Allocator,
    frame_index: usize,
    swapchain_extent: vk::Extent2D,
) -> Result<FrameRenderer> {
    //------// commands //------//

    // command pool
    let command_pool_create_info = vk::CommandPoolCreateInfo::default()
        .queue_family_index(device.queue_family_indices(QueueRoleFlags::RENDER)[0]);
    let command_pool = unsafe { device.create_command_pool(&command_pool_create_info, None) }?;

    // command buffers allocation
    let render_allocate_info = vk::CommandBufferAllocateInfo::default()
        .command_pool(command_pool)
        .command_buffer_count(1)
        .level(vk::CommandBufferLevel::PRIMARY);
    let command_buffers = unsafe { device.allocate_command_buffers(&render_allocate_info) }?;

    //------// sync //------//

    // swapchain_image_available_binary
    let swapchain_image_available_binary = device.create_binary_short()?;
    device.name(
        swapchain_image_available_binary,
        CString::new(format!(
            "frame_renderer_{frame_index} - swapchain_image_available_binary"
        ))?,
    )?;

    // rendering_done_fence
    let rendering_done_fence = device.create_fence_short(true)?;
    device.name(
        rendering_done_fence,
        CString::new(format!(
            "frame_renderer_{frame_index} - rendering_done_fence"
        ))?,
    )?;

    //------// resources //------//

    // depth
    let depth = Depth::create(instance, device, allocator, swapchain_extent)?;

    Ok(FrameRenderer {
        frame_index,
        command_pool,
        render: command_buffers[0],
        swapchain_image_available_binary,
        rendering_done_fence,
        depth,
    })
}
