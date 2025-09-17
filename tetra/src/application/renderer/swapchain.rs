mod create;

use std::ops::Deref;

use ash::vk;

use crate::context::{Device, Instance};

use super::FRAMES_IN_FLIGHT;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////

pub struct Swapchain {
    swapchain: vk::SwapchainKHR,
    pub images: Vec<vk::Image>,
    pub image_views: Vec<vk::ImageView>,
    pub image_presentable_binaries: Vec<vk::Semaphore>,

    // infos
    pub extent: vk::Extent2D,
    pub format: vk::Format,
    pub subresource_range: vk::ImageSubresourceRange,
}

/////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////

/// Deref : `vk::SwapchainKHR`
impl Deref for Swapchain {
    type Target = vk::SwapchainKHR;

    fn deref(&self) -> &Self::Target {
        &self.swapchain
    }
}

/// Create & Recreate & Destroy
impl Swapchain {
    pub fn create(instance: &Instance, device: &Device) -> Result<Self> {
        create::create(instance, device)
    }

    pub fn recreate(&mut self, instance: &Instance, device: &Device) -> Result<()> {
        create::recreate(self, instance, device)
    }

    pub fn destroy(&mut self, device: &Device) {
        unsafe {
            for binary in &self.image_presentable_binaries {
                device.destroy_semaphore(*binary, None);
            }
            for image_view in &self.image_views {
                device.destroy_image_view(*image_view, None);
            }
            device.swapchain_device.destroy_swapchain(**self, None);
        }
    }
}
