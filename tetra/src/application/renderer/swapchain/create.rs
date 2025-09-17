mod create_from_configuration;

use ash::vk;

use crate::context::{Device, Instance, device::QueueRoleFlags};

use super::{FRAMES_IN_FLIGHT, Swapchain};

use create_from_configuration::{configuration::*, create_from_configuration};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

const SWAPCHAIN_SURFACE_FORMAT: vk::SurfaceFormatKHR = vk::SurfaceFormatKHR {
    format: vk::Format::B8G8R8A8_UNORM,
    color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
};
const SWAPCHAIN_USAGE: vk::FormatFeatureFlags2 = vk::FormatFeatureFlags2::COLOR_ATTACHMENT;
const SWAPCHAIN_MIN_IMAGE_COUNT: u32 = FRAMES_IN_FLIGHT as u32 + 1;

/////////////////////////////////////////////////////////////////////////////

fn configure(instance: &Instance, device: &Device) -> Result<SwapchainConfiguration> {
    // extract
    let surface_capabilities = unsafe {
        instance
            .surface_instance
            .get_physical_device_surface_capabilities(device.physical_device, device.surface)
    }?;
    let surface_formats = unsafe {
        instance
            .surface_instance
            .get_physical_device_surface_formats(device.physical_device, device.surface)
    }?;

    // check
    instance.physical_device_support_format_usage_in_optimal_tiling(
        device.physical_device,
        SWAPCHAIN_SURFACE_FORMAT.format,
        SWAPCHAIN_USAGE,
    )?;
    if !surface_formats.contains(&SWAPCHAIN_SURFACE_FORMAT) {
        return Err("swapchain surface format not supported".into());
    }
    if surface_capabilities.max_image_count < SWAPCHAIN_MIN_IMAGE_COUNT {
        return Err("swapchain minimum image count not supported".into());
    }

    // configure
    Ok(SwapchainConfiguration {
        surface: SurfaceConfiguration {
            surface: device.surface,
            pre_transform: surface_capabilities.current_transform,
            image_extent: surface_capabilities.current_extent,
            image_format: SWAPCHAIN_SURFACE_FORMAT.format,
            image_color_space: SWAPCHAIN_SURFACE_FORMAT.color_space,
        },
        swap: SwapConfiguration {
            min_image_count: SWAPCHAIN_MIN_IMAGE_COUNT,
            present_mode: vk::PresentModeKHR::FIFO,
        },
        usage: UsageConfiguration {
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            queue_family_indices: device.queue_family_indices(QueueRoleFlags::PRESENT),
            image_sharing_mode: vk::SharingMode::EXCLUSIVE,
        },
        boilerplate: BoilerplateConfiguration::default(),
    })
}

pub fn create(instance: &Instance, device: &Device) -> Result<Swapchain> {
    let configuration = configure(instance, device)?;
    create_from_configuration(device, configuration, None)
}

pub fn recreate(swapchain: &mut Swapchain, instance: &Instance, device: &Device) -> Result<()> {
    let configuration = configure(instance, device)?;
    let new_swapchain = create_from_configuration(device, configuration, Some(**swapchain))?;
    swapchain.destroy(device);
    *swapchain = new_swapchain;
    Ok(())
}
