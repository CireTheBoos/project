use std::ffi::{CStr, c_char};

use ash::vk;
use vk_utils::Features;

use super::{
    Device, Instance,
    queues::{Queue, QueueRoleFlags},
};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////
// Argument
/////////////////////////////////////////////////////////////////////////

pub mod configuration {
    use super::*;

    #[derive(Debug)]
    pub struct DeviceConfiguration {
        // configuration
        pub extension_names: Vec<&'static CStr>,
        pub features: Features,
        pub queue_families: Vec<QueueFamilyConfiguration>,

        // arguments
        pub physical_device: vk::PhysicalDevice,
        pub surface: vk::SurfaceKHR,
    }

    /// SoA instead of AoS for queues because creation need priorities as `&[f32]`.
    #[derive(Debug)]
    pub struct QueueFamilyConfiguration {
        pub family_index: u32,
        pub priorities: Vec<f32>,
        pub role_flags: Vec<QueueRoleFlags>,
    }
}

/////////////////////////////////////////////////////////////////////////
// Function
/////////////////////////////////////////////////////////////////////////

pub fn create_from_configuration(
    instance: &Instance,
    mut configuration: configuration::DeviceConfiguration,
) -> Result<Device> {
    // extensions
    let enabled_extension_names: Vec<*const c_char> = configuration
        .extension_names
        .into_iter()
        .map(|extension| extension.as_ptr())
        .collect();

    // features
    let mut enabled_features = vk::PhysicalDeviceFeatures2::default()
        .features(configuration.features.features_10)
        .push_next(&mut configuration.features.features_11)
        .push_next(&mut configuration.features.features_12)
        .push_next(&mut configuration.features.features_13);

    // check all roles are assigned
    let mut total_roles = QueueRoleFlags::empty();
    for queue_family in &configuration.queue_families {
        for queue_role in &queue_family.role_flags {
            total_roles |= *queue_role;
        }
    }
    if !total_roles.contains(QueueRoleFlags::ALL) {
        return Err("some queue roles are not assigned".into());
    }

    // queue families
    let queue_family_create_infos: Vec<vk::DeviceQueueCreateInfo> = configuration
        .queue_families
        .iter()
        .map(|queue_family| {
            vk::DeviceQueueCreateInfo::default()
                .queue_family_index(queue_family.family_index)
                .queue_priorities(&queue_family.priorities)
        })
        .collect();

    // create info
    let create_info = vk::DeviceCreateInfo::default()
        .queue_create_infos(&queue_family_create_infos)
        .enabled_extension_names(&enabled_extension_names)
        .push_next(&mut enabled_features);

    // loaders
    let device =
        unsafe { instance.create_device(configuration.physical_device, &create_info, None) }?;
    let swapchain_device = ash::khr::swapchain::Device::new(instance, &device);
    let debug_utils_device = ash::ext::debug_utils::Device::new(instance, &device);

    // queues
    let mut queues = Vec::new();
    for queue_family in configuration.queue_families {
        let family_index = queue_family.family_index;
        for (index, roles) in queue_family.role_flags.into_iter().enumerate() {
            queues.push(Queue {
                family_index,
                vk_queue: unsafe { device.get_device_queue(family_index, index as u32) },
                roles,
            });
        }
    }

    Ok(Device {
        // loaders
        device,
        swapchain_device,
        debug_utils_device,

        // queues
        queues,

        // arguments
        physical_device: configuration.physical_device,
        surface: configuration.surface,
    })
}
