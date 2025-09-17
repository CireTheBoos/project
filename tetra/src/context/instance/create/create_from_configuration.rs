use std::ffi::{CStr, c_char};

use ash::vk;
use vk_utils::ApiVersion;

use super::{Entry, Instance};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////
// Argument
/////////////////////////////////////////////////////////////////////////

pub mod configuration {
    use super::*;

    #[derive(Debug)]
    pub struct InstanceConfiguration {
        pub application: ApplicationConfiguration,
        pub layer_names: Vec<&'static CStr>,
        pub extension_names: Vec<&'static CStr>,
    }

    #[derive(Debug)]
    pub struct ApplicationConfiguration {
        pub api_version: ApiVersion,
        pub name: &'static CStr,
    }
}

/////////////////////////////////////////////////////////////////////////
// Function
/////////////////////////////////////////////////////////////////////////

pub fn create_from_configuration(
    entry: &Entry,
    configuration: configuration::InstanceConfiguration,
) -> Result<Instance> {
    // application info
    let application_info = vk::ApplicationInfo::default()
        .api_version(configuration.application.api_version.into())
        .application_name(configuration.application.name);

    // layers
    let enabled_layer_names: Vec<*const c_char> = configuration
        .layer_names
        .into_iter()
        .map(|layer| layer.as_ptr())
        .collect();

    // extensions
    let enabled_extension_names: Vec<*const c_char> = configuration
        .extension_names
        .into_iter()
        .map(|extension| extension.as_ptr())
        .collect();

    // create info
    let create_info = vk::InstanceCreateInfo::default()
        .application_info(&application_info)
        .enabled_layer_names(&enabled_layer_names)
        .enabled_extension_names(&enabled_extension_names);

    // loaders
    let instance = unsafe { entry.create_instance(&create_info, None) }?;
    let surface_instance = ash::khr::surface::Instance::new(entry, &instance);

    Ok(Instance {
        instance,
        surface_instance,
    })
}
