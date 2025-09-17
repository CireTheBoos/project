use std::ffi::CStr;

use ash::vk;
use vk_utils::{ApiVersion, Features};

use super::Instance;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////

pub fn api_version(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    api_version: ApiVersion,
) -> Result<()> {
    // extract
    let physical_device_properties =
        unsafe { instance.get_physical_device_properties(physical_device) };
    let max_supported_api_version = ApiVersion::from(physical_device_properties.api_version);

    // check
    if api_version > max_supported_api_version {
        return Err("physical device do not support api version".into());
    }

    Ok(())
}

pub fn extensions(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    extension_names: &[&CStr],
) -> Result<()> {
    // extract
    let supported_extensions: Vec<vk::ExtensionProperties> =
        unsafe { instance.enumerate_device_extension_properties(physical_device)? };
    let supported_extension_names: Vec<&CStr> = supported_extensions
        .iter()
        .map(|supported_extension| supported_extension.extension_name_as_c_str().unwrap())
        .collect();

    // check
    for extension_name in extension_names {
        if !supported_extension_names.contains(extension_name) {
            return Err("physical device do not support extension".into());
        }
    }

    Ok(())
}

pub fn features(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    features: &Features,
) -> Result<()> {
    // extract
    let mut supported_features = Features::default();
    let mut features2 = vk::PhysicalDeviceFeatures2::default()
        .features(vk::PhysicalDeviceFeatures::default())
        .push_next(&mut supported_features.features_11)
        .push_next(&mut supported_features.features_12)
        .push_next(&mut supported_features.features_13);
    unsafe { instance.get_physical_device_features2(physical_device, &mut features2) };
    supported_features.features_10 = features2.features;

    // check
    if !supported_features.contains(features) {
        return Err("physical device do not support feature".into());
    }

    Ok(())
}

pub fn format_usage_in_optimal_tiling(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    format: vk::Format,
    usage: vk::FormatFeatureFlags2,
) -> Result<()> {
    // extract
    let mut format_properties = vk::FormatProperties3::default();
    let mut format_properties2 = vk::FormatProperties2::default().push_next(&mut format_properties);
    unsafe {
        instance.get_physical_device_format_properties2(
            physical_device,
            format,
            &mut format_properties2,
        )
    };

    // check
    if !format_properties.optimal_tiling_features.contains(usage) {
        return Err("physical device do not support format usage in optimal tiling".into());
    }

    Ok(())
}
