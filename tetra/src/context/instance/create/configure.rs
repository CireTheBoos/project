use std::ffi::CStr;

use ash::vk;
use vk_utils::ApiVersion;
use winit::raw_window_handle::RawDisplayHandle;

use super::{Entry, configuration::*};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

const INSTANCE_API_VERSION: ApiVersion = ApiVersion::from_const(vk::API_VERSION_1_3);
const APPLICATION_NAME: &CStr = c"Tetra";
const VALIDATION_LAYER: &CStr = c"VK_LAYER_KHRONOS_validation";

/////////////////////////////////////////////////////////////////////////
// Function
/////////////////////////////////////////////////////////////////////////

pub fn configure(entry: &Entry, display_handle: RawDisplayHandle) -> Result<InstanceConfiguration> {
    let application = configure_application(entry)?;
    let layer_names = configure_layers(entry)?;
    let extension_names = configure_extensions(entry, display_handle)?;

    Ok(InstanceConfiguration {
        application,
        layer_names,
        extension_names,
    })
}

/////////////////////////////////////////////////////////////////////////
// Sub functions
/////////////////////////////////////////////////////////////////////////

fn configure_application(entry: &Entry) -> Result<ApplicationConfiguration> {
    //-------------// Required //-------------//

    // define
    let required_application = ApplicationConfiguration {
        api_version: INSTANCE_API_VERSION,
        name: APPLICATION_NAME,
    };

    // check
    entry.instance_support_api_version(required_application.api_version)?;

    //-------------//

    Ok(required_application)
}

fn configure_layers(entry: &Entry) -> Result<Vec<&'static CStr>> {
    //-------------// Required //-------------//

    // define
    let required_layers = vec![VALIDATION_LAYER];

    // check
    entry.instance_support_layers(&required_layers)?;

    //-------------//

    Ok(required_layers)
}

fn configure_extensions(
    entry: &Entry,
    display_handle: RawDisplayHandle,
) -> Result<Vec<&'static CStr>> {
    //-------------// Required //-------------//

    // define
    let mut required_extensions = vec![vk::EXT_DEBUG_UTILS_NAME];
    let mut required_display_extensions =
        ash_window::enumerate_required_extensions(display_handle)?
            .iter()
            .map(|ext_ptr| unsafe { CStr::from_ptr(*ext_ptr) })
            .collect();
    required_extensions.append(&mut required_display_extensions);

    // check
    entry.instance_support_extensions(&required_extensions)?;

    //-------------//

    Ok(required_extensions)
}
