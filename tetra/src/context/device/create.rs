mod configure;
mod create_from_configuration;
mod evaluate_configuration;

use ash::vk;

use super::{Device, Instance, queues};

use configure::configure;
use create_from_configuration::{
    configuration::{self, DeviceConfiguration},
    create_from_configuration,
};
use evaluate_configuration::evaluate_configuration;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////

pub fn create(instance: &Instance, surface: vk::SurfaceKHR) -> Result<Device> {
    // create arguments : surface + physical devices
    let physical_devices = unsafe { instance.enumerate_physical_devices() }?;

    // configure each physical devices
    let device_configuration_results: Vec<Result<DeviceConfiguration>> = physical_devices
        .into_iter()
        .map(|physical_device| configure(&instance, physical_device, surface))
        .collect();

    // filter out failed configurations
    let device_configurations: Vec<DeviceConfiguration> = device_configuration_results
        .into_iter()
        .filter_map(|configuration_result| configuration_result.ok())
        .collect();

    // select best configuration
    let best_device_configuration: DeviceConfiguration = device_configurations
        .into_iter()
        .max_by_key(|device_configuration| evaluate_configuration(instance, &device_configuration))
        .ok_or::<Error>("no suitable physical device".into())?;

    // create
    Ok(create_from_configuration(
        &instance,
        best_device_configuration,
    )?)
}
