use std::ffi::CStr;

use ash::vk;
use vk_utils::{ApiVersion, Features};

use super::{Instance, configuration::*, queues::QueueRoleFlags};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

const DEVICE_API_VERSION: ApiVersion = ApiVersion::from_const(vk::API_VERSION_1_3);

/////////////////////////////////////////////////////////////////////////
// Function
/////////////////////////////////////////////////////////////////////////

pub fn configure(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
) -> Result<DeviceConfiguration> {
    // check
    instance.physical_device_support_api_version(physical_device, DEVICE_API_VERSION)?;

    // configure
    let extension_names = configure_extensions(instance, physical_device)?;
    let features = configure_features(instance, physical_device)?;
    let queue_families = configure_queue_families(instance, physical_device, surface)?;

    Ok(DeviceConfiguration {
        // configuration
        extension_names,
        features,
        queue_families,

        // arguments
        physical_device,
        surface,
    })
}

/////////////////////////////////////////////////////////////////////////
// Sub functions
/////////////////////////////////////////////////////////////////////////

fn configure_extensions(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
) -> Result<Vec<&'static CStr>> {
    //-------------// Required //-------------//

    // define
    let required_extensions = vec![vk::KHR_SWAPCHAIN_NAME];

    // check
    instance.physical_device_support_extensions(physical_device, &required_extensions)?;

    //-------------//

    Ok(required_extensions)
}

fn configure_features(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
) -> Result<Features> {
    //-------------// Required //-------------//

    // define
    let required_features = Features {
        features_10: vk::PhysicalDeviceFeatures::default()
            .geometry_shader(true)
            .multi_draw_indirect(true),
        features_11: vk::PhysicalDeviceVulkan11Features::default(),
        features_12: vk::PhysicalDeviceVulkan12Features::default(),
        features_13: vk::PhysicalDeviceVulkan13Features::default()
            .dynamic_rendering(true)
            .synchronization2(true),
    };

    // check
    instance.physical_device_support_features(physical_device, &required_features)?;

    //-------------//

    Ok(required_features)
}

fn configure_queue_families(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
) -> Result<Vec<QueueFamilyConfiguration>> {
    //-------------// Required //-------------//

    // define/check : Single queue
    let single_queue_qfi = find_queue_family_index(
        instance,
        physical_device,
        QueueFamilyDescription {
            with_flags: vk::QueueFlags::GRAPHICS
                | vk::QueueFlags::COMPUTE
                | vk::QueueFlags::TRANSFER,
            without_flags: vk::QueueFlags::empty(),
            with_surface_support: Some(surface),
        },
    )?;

    // configure
    let single_queue_configuration = vec![QueueFamilyConfiguration {
        family_index: single_queue_qfi,
        priorities: vec![1.],
        role_flags: vec![QueueRoleFlags::ALL],
    }];

    //-------------// Prefered //-------------//

    // define : Dedicated queue families (distinct due to `without_flags`)
    let graphics_qfi = find_queue_family_index(
        instance,
        physical_device,
        QueueFamilyDescription {
            with_flags: vk::QueueFlags::GRAPHICS,
            without_flags: vk::QueueFlags::empty(),
            with_surface_support: Some(surface),
        },
    );
    let compute_qfi = find_queue_family_index(
        instance,
        physical_device,
        QueueFamilyDescription {
            with_flags: vk::QueueFlags::COMPUTE,
            without_flags: vk::QueueFlags::GRAPHICS,
            with_surface_support: None,
        },
    );
    let transfer_qfi = find_queue_family_index(
        instance,
        physical_device,
        QueueFamilyDescription {
            with_flags: vk::QueueFlags::TRANSFER,
            without_flags: vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE,
            with_surface_support: None,
        },
    );

    // check/configure
    let dedicated_queue_families_configuration =
        if graphics_qfi.is_ok() && compute_qfi.is_ok() && transfer_qfi.is_ok() {
            let graphics_family = QueueFamilyConfiguration {
                family_index: graphics_qfi.unwrap(),
                priorities: vec![1.],
                role_flags: vec![QueueRoleFlags::GRAPHICS],
            };
            let compute_family = QueueFamilyConfiguration {
                family_index: compute_qfi.unwrap(),
                priorities: vec![1.],
                role_flags: vec![QueueRoleFlags::COMPUTE],
            };
            let transfer_family = QueueFamilyConfiguration {
                family_index: transfer_qfi.unwrap(),
                priorities: vec![1.],
                role_flags: vec![QueueRoleFlags::TRANSFER],
            };

            Some(vec![graphics_family, compute_family, transfer_family])
        } else {
            None
        };

    Ok(dedicated_queue_families_configuration.unwrap_or(single_queue_configuration))
}

struct QueueFamilyDescription {
    pub with_flags: vk::QueueFlags,
    pub without_flags: vk::QueueFlags,
    pub with_surface_support: Option<vk::SurfaceKHR>,
}

fn find_queue_family_index(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    description: QueueFamilyDescription,
) -> Result<u32> {
    // extract
    let queue_family_properties =
        unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

    // search
    let matching_queue_families: Vec<u32> = queue_family_properties
        .into_iter()
        .enumerate()
        .filter_map(|(qfi, _)| {
            // extract flags support
            let support_flags = instance
                .physical_device_queue_family_support_flags(
                    physical_device,
                    qfi as u32,
                    description.with_flags,
                    description.without_flags,
                )
                .is_ok();

            // extract surface support
            let support_surface = description
                .with_surface_support
                .map(|surface| {
                    instance
                        .physical_device_queue_family_support_surface(
                            physical_device,
                            qfi as u32,
                            surface,
                        )
                        .is_ok()
                })
                .unwrap_or(true);

            // check
            (support_flags && support_surface).then_some(qfi as u32)
        })
        .collect();

    matching_queue_families
        .first()
        .copied()
        .ok_or("no appropriate queue family found".into())
}
