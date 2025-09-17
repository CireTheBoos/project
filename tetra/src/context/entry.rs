mod instance_support;

use std::{ffi::CStr, ops::Deref};

use vk_utils::ApiVersion;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////

/// Provide additional functions :
/// - `instance_support_..`
pub struct Entry {
    // loader
    entry: ash::Entry,
}

/////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////

/// Deref : `ash::Entry`
impl Deref for Entry {
    type Target = ash::Entry;

    fn deref(&self) -> &Self::Target {
        &self.entry
    }
}

/// New
impl Entry {
    pub fn new() -> Result<Self> {
        Ok(Self {
            entry: unsafe { ash::Entry::load() }?,
        })
    }
}

/// Support
impl Entry {
    pub fn instance_support_api_version(&self, api_version: ApiVersion) -> Result<()> {
        instance_support::api_version(self, api_version)
    }

    pub fn instance_support_layers(&self, layer_names: &[&CStr]) -> Result<()> {
        instance_support::layers(self, layer_names)
    }

    pub fn instance_support_extensions(&self, extension_names: &[&CStr]) -> Result<()> {
        instance_support::extensions(self, extension_names)
    }
}
