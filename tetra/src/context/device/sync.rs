use ash::{prelude::VkResult, vk};

use super::{Device, QueueRoleFlags};

/////////////////////////////////////////////////////////////////////////
// Create functions
/////////////////////////////////////////////////////////////////////////

pub fn create_timeline_short(device: &Device, initial_value: u64) -> VkResult<vk::Semaphore> {
    let mut type_create_info = vk::SemaphoreTypeCreateInfo::default()
        .semaphore_type(vk::SemaphoreType::TIMELINE)
        .initial_value(initial_value);
    let create_info = vk::SemaphoreCreateInfo::default().push_next(&mut type_create_info);
    unsafe { device.create_semaphore(&create_info, None) }
}

pub fn create_binary_short(device: &Device) -> VkResult<vk::Semaphore> {
    let mut type_create_info =
        vk::SemaphoreTypeCreateInfo::default().semaphore_type(vk::SemaphoreType::BINARY);
    let create_info = vk::SemaphoreCreateInfo::default().push_next(&mut type_create_info);
    unsafe { device.create_semaphore(&create_info, None) }
}

pub fn create_fence_short(device: &Device, already_signaled: bool) -> VkResult<vk::Fence> {
    let create_flags = if already_signaled {
        vk::FenceCreateFlags::SIGNALED
    } else {
        vk::FenceCreateFlags::empty()
    };
    let create_info = vk::FenceCreateInfo::default().flags(create_flags);
    unsafe { device.create_fence(&create_info, None) }
}

/////////////////////////////////////////////////////////////////////////
// Swapchain functions
/////////////////////////////////////////////////////////////////////////

pub fn queue_present_single_swapchain(
    device: &Device,
    swapchain: vk::SwapchainKHR,
    swapchain_image_index: u32,
    wait_binary_semaphores: &[vk::Semaphore],
) -> VkResult<bool> {
    let swapchains = [swapchain];
    let image_indices = [swapchain_image_index];
    let present_info = vk::PresentInfoKHR::default()
        .swapchains(&swapchains)
        .image_indices(&image_indices)
        .wait_semaphores(&wait_binary_semaphores);
    unsafe {
        device.swapchain_device.queue_present(
            device.queue(QueueRoleFlags::PRESENT).vk_queue,
            &present_info,
        )
    }
}
