use ash::vk;

/////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////

/// - Comparable.
/// - Conversion from & into `u32`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ApiVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub variant: u32,
}

/////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////

/// From u32
impl From<u32> for ApiVersion {
    fn from(version_u32: u32) -> Self {
        ApiVersion {
            major: vk::api_version_major(version_u32),
            minor: vk::api_version_minor(version_u32),
            patch: vk::api_version_patch(version_u32),
            variant: vk::api_version_variant(version_u32),
        }
    }
}

// From u32 (const)
impl ApiVersion {
    pub const fn from_const(version_u32: u32) -> Self {
        ApiVersion {
            major: vk::api_version_major(version_u32),
            minor: vk::api_version_minor(version_u32),
            patch: vk::api_version_patch(version_u32),
            variant: vk::api_version_variant(version_u32),
        }
    }
}

/// To u32
impl From<ApiVersion> for u32 {
    fn from(version_struct: ApiVersion) -> Self {
        vk::make_api_version(
            version_struct.variant,
            version_struct.major,
            version_struct.minor,
            version_struct.patch,
        )
    }
}
