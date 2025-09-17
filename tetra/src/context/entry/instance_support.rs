use std::ffi::CStr;

use ash::vk;
use vk_utils::ApiVersion;

use super::Entry;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////

pub fn api_version(entry: &Entry, api_version: ApiVersion) -> Result<()> {
    // extract
    let max_supported_api_version = ApiVersion::from(
        unsafe { entry.try_enumerate_instance_version()? }.unwrap_or(vk::API_VERSION_1_0),
    );

    // check
    if api_version > max_supported_api_version {
        return Err("instance do not support api version".into());
    }

    Ok(())
}

pub fn layers(entry: &Entry, layer_names: &[&CStr]) -> Result<()> {
    // extract
    let supported_layers: Vec<vk::LayerProperties> =
        unsafe { entry.enumerate_instance_layer_properties()? };
    let supported_layer_names: Vec<&CStr> = supported_layers
        .iter()
        .map(|supported_layer| supported_layer.layer_name_as_c_str().unwrap())
        .collect();

    // check
    for layer_name in layer_names {
        if !supported_layer_names.contains(layer_name) {
            return Err("instance do not support layer".into());
        }
    }

    Ok(())
}

pub fn extensions(entry: &Entry, extension_names: &[&CStr]) -> Result<()> {
    // extract
    let supported_extensions: Vec<vk::ExtensionProperties> =
        unsafe { entry.enumerate_instance_extension_properties(None)? };
    let supported_extension_names: Vec<&CStr> = supported_extensions
        .iter()
        .map(|supported_extension| supported_extension.extension_name_as_c_str().unwrap())
        .collect();

    // check
    for extension_name in extension_names {
        if !supported_extension_names.contains(extension_name) {
            return Err("instance do not support extension".into());
        }
    }

    Ok(())
}
