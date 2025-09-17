use ash::vk;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct DepthStencilConfiguration {
    // depth
    pub depth_test_enable: bool,
    pub depth_compare_op: vk::CompareOp,
    pub depth_write_enable: bool,

    // depth bounds
    pub depth_bounds_test_enable: bool,
    pub min_depth_bounds: f32,
    pub max_depth_bounds: f32,

    // stencil
    pub stencil_test_enable: bool,
    pub back: vk::StencilOpState,
    pub front: vk::StencilOpState,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Constructors
impl DepthStencilConfiguration {
    /// - Depth test enabled :
    ///     - Compare op = `LESS`.
    ///     - Depth write enabled.
    /// - No depths bounds test.
    /// - No stencil test.
    pub fn test_depth_less_and_overwrite_on_success() -> Self {
        Self {
            depth_test_enable: true,
            depth_compare_op: vk::CompareOp::LESS,
            depth_write_enable: true,
            depth_bounds_test_enable: false,
            min_depth_bounds: Default::default(),
            max_depth_bounds: Default::default(),
            stencil_test_enable: false,
            back: Default::default(),
            front: Default::default(),
        }
    }
}
