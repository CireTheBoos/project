use std::ffi::CString;

use ash::vk;

use super::{Device, Swapchain};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////////
// Function
/////////////////////////////////////////////////////////////////////////////

pub fn create_from_configuration(
    device: &Device,
    configuration: configuration::SwapchainConfiguration,
    old_swapchain: Option<vk::SwapchainKHR>,
) -> Result<Swapchain> {
    // swapchain
    let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
        // (old swapchain)
        .old_swapchain(old_swapchain.unwrap_or(vk::SwapchainKHR::null()))
        // surface
        .surface(configuration.surface.surface)
        .pre_transform(configuration.surface.pre_transform)
        .image_extent(configuration.surface.image_extent)
        .image_format(configuration.surface.image_format)
        .image_color_space(configuration.surface.image_color_space)
        // swap
        .present_mode(configuration.swap.present_mode)
        .min_image_count(configuration.swap.min_image_count)
        // usage
        .image_usage(configuration.usage.image_usage)
        .queue_family_indices(&configuration.usage.queue_family_indices)
        .image_sharing_mode(configuration.usage.image_sharing_mode)
        // boilerplate
        .image_array_layers(configuration.boilerplate.image_array_layers)
        .clipped(configuration.boilerplate.clipped)
        .composite_alpha(configuration.boilerplate.composite_alpha);
    let swapchain = unsafe {
        device
            .swapchain_device
            .create_swapchain(&swapchain_create_info, None)?
    };

    // images
    let images = unsafe { device.swapchain_device.get_swapchain_images(swapchain) }?;

    // image views
    let subresource_range = vk::ImageSubresourceRange::default()
        .aspect_mask(vk::ImageAspectFlags::COLOR)
        .layer_count(1)
        .level_count(1);
    let mut image_views = Vec::with_capacity(images.len());
    for image in &images {
        let create_info = vk::ImageViewCreateInfo::default()
            .image(*image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(configuration.surface.image_format)
            .subresource_range(subresource_range)
            .components(vk::ComponentMapping::default());
        let image_view = unsafe { device.create_image_view(&create_info, None) }?;
        image_views.push(image_view);
    }

    // presentable_binaries
    let mut image_presentable_binaries = Vec::with_capacity(images.len());
    for image_index in 0..images.len() {
        let image_presentable_binary = device.create_binary_short()?;
        device.name(
            image_presentable_binary,
            CString::new(format!(
                "swapchain - image_{image_index}_presentable_binary"
            ))?,
        )?;
        image_presentable_binaries.push(image_presentable_binary);
    }

    Ok(Swapchain {
        swapchain,
        images,
        image_views,
        image_presentable_binaries,
        extent: configuration.surface.image_extent,
        format: configuration.surface.image_format,
        subresource_range,
    })
}

/////////////////////////////////////////////////////////////////////////////
// Configuration structures
/////////////////////////////////////////////////////////////////////////////

pub mod configuration {
    use super::*;

    #[derive(Debug)]
    pub struct SwapchainConfiguration {
        pub surface: SurfaceConfiguration,
        pub swap: SwapConfiguration,
        pub usage: UsageConfiguration,
        pub boilerplate: BoilerplateConfiguration,
    }

    #[derive(Debug)]
    pub struct SurfaceConfiguration {
        pub surface: vk::SurfaceKHR,
        pub pre_transform: vk::SurfaceTransformFlagsKHR,
        pub image_extent: vk::Extent2D,
        pub image_format: vk::Format,
        pub image_color_space: vk::ColorSpaceKHR,
    }

    #[derive(Debug)]
    pub struct SwapConfiguration {
        pub min_image_count: u32,
        pub present_mode: vk::PresentModeKHR,
    }

    #[derive(Debug)]
    pub struct UsageConfiguration {
        pub image_usage: vk::ImageUsageFlags,
        pub queue_family_indices: Vec<u32>,
        pub image_sharing_mode: vk::SharingMode,
    }

    #[derive(Debug)]
    pub struct BoilerplateConfiguration {
        pub image_array_layers: u32,
        pub clipped: bool,
        pub composite_alpha: vk::CompositeAlphaFlagsKHR,
    }

    impl Default for BoilerplateConfiguration {
        fn default() -> Self {
            Self {
                image_array_layers: 1,
                clipped: true,
                composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
            }
        }
    }
}
