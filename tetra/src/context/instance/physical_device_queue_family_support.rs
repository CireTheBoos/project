use ash::vk;

use super::Instance;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////

pub fn surface(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    queue_family_index: u32,
    surface: vk::SurfaceKHR,
) -> Result<()> {
    // extract
    let support_surface = unsafe {
        instance
            .surface_instance
            .get_physical_device_surface_support(physical_device, queue_family_index, surface)?
    };

    // check
    if !support_surface {
        return Err("physical device queue family do not support surface".into());
    }

    Ok(())
}

pub fn flags(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    queue_family_index: u32,
    requested_flags: vk::QueueFlags,
    forbidden_flags: vk::QueueFlags,
) -> Result<()> {
    // extract
    let queue_family_properties =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
    let queue_flags = queue_family_properties[queue_family_index as usize].queue_flags;

    // check
    if !queue_flags.contains(requested_flags) {
        return Err("physical device queue family do not have requested flags".into());
    }
    if queue_flags.intersects(forbidden_flags) {
        return Err("physical device queue family have forbidden flags".into());
    }

    Ok(())
}
