mod create;
mod physical_device_queue_family_support;
mod physical_device_support;

use std::{ffi::CStr, ops::Deref};

use ash::vk;
use vk_utils::{ApiVersion, Features};
use winit::raw_window_handle::RawDisplayHandle;

use super::Entry;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////

/// - Wrap loaders.
/// - Provide additional functions :
///     - `physical_device_support_..`
///     - `physical_device_queue_family_support_..`
///     - No additional functions to query data (they are simple enough).
pub struct Instance {
    // loaders
    instance: ash::Instance,
    pub surface_instance: ash::khr::surface::Instance,
}

/////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////

/// Deref : `ash::Instance`
impl Deref for Instance {
    type Target = ash::Instance;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

/// Create & Destroy
impl Instance {
    pub fn create(entry: &Entry, display_handle: RawDisplayHandle) -> Result<Self> {
        create::create(entry, display_handle)
    }

    pub fn destroy(&mut self) {
        unsafe { self.destroy_instance(None) };
    }
}

/// Support
impl Instance {
    pub fn physical_device_support_api_version(
        &self,
        physical_device: vk::PhysicalDevice,
        api_version: ApiVersion,
    ) -> Result<()> {
        physical_device_support::api_version(self, physical_device, api_version)
    }

    pub fn physical_device_support_extensions(
        &self,
        physical_device: vk::PhysicalDevice,
        extension_names: &[&CStr],
    ) -> Result<()> {
        physical_device_support::extensions(self, physical_device, extension_names)
    }

    pub fn physical_device_support_features(
        &self,
        physical_device: vk::PhysicalDevice,
        features: &Features,
    ) -> Result<()> {
        physical_device_support::features(self, physical_device, features)
    }

    pub fn physical_device_support_format_usage_in_optimal_tiling(
        &self,
        physical_device: vk::PhysicalDevice,
        format: vk::Format,
        usage: vk::FormatFeatureFlags2,
    ) -> Result<()> {
        physical_device_support::format_usage_in_optimal_tiling(
            self,
            physical_device,
            format,
            usage,
        )
    }

    pub fn physical_device_queue_family_support_surface(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        surface: vk::SurfaceKHR,
    ) -> Result<()> {
        physical_device_queue_family_support::surface(
            self,
            physical_device,
            queue_family_index,
            surface,
        )
    }

    pub fn physical_device_queue_family_support_flags(
        &self,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
        requested_flags: vk::QueueFlags,
        forbidden_flags: vk::QueueFlags,
    ) -> Result<()> {
        physical_device_queue_family_support::flags(
            self,
            physical_device,
            queue_family_index,
            requested_flags,
            forbidden_flags,
        )
    }
}
