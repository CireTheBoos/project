use ash::vk;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct InputAssemblyConfiguration {
    pub topology: vk::PrimitiveTopology,
    pub primitive_restart_enable: bool,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Constructors
impl InputAssemblyConfiguration {
    /// - `topology` = `TRIANGLE_LIST`.
    /// - `primitive_restart_enable` = `false`.
    pub fn triangle_list() -> InputAssemblyConfiguration {
        InputAssemblyConfiguration {
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            primitive_restart_enable: false,
        }
    }
}
