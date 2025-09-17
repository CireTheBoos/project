use ash::vk;

use super::{Instance, configuration::*};

/////////////////////////////////////////////////////////////////////////

pub fn evaluate_configuration(instance: &Instance, configuration: &DeviceConfiguration) -> u32 {
    // extract
    let physical_device_properties =
        unsafe { instance.get_physical_device_properties(configuration.physical_device) };

    // evaluate
    if physical_device_properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
        1
    } else {
        0
    }
}
