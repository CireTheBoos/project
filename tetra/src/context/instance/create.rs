mod configure;
mod create_from_configuration;

use winit::raw_window_handle::RawDisplayHandle;

use super::{Entry, Instance};

use configure::configure;
use create_from_configuration::{configuration, create_from_configuration};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////

pub fn create(entry: &Entry, display_handle: RawDisplayHandle) -> Result<Instance> {
    let instance_configuration = configure(&entry, display_handle)?;
    Ok(create_from_configuration(&entry, instance_configuration)?)
}
