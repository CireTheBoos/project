use ash::vk;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct RasterizationConfiguration {
    // rasterization
    pub rasterizer_discard_enable: bool,
    pub polygon_mode: vk::PolygonMode,
    pub line_width: f32,

    // culling
    pub cull_mode: vk::CullModeFlags,
    pub front_face: vk::FrontFace,

    // depth clamp
    pub depth_clamp_enable: bool,

    // depth bias
    pub depth_bias_enable: bool,
    pub depth_bias_constant_factor: f32,
    pub depth_bias_slope_factor: f32,
    pub depth_bias_clamp: f32,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Constructors
impl RasterizationConfiguration {
    /// - Rasterize fill.
    /// - Cull back faces with front = `CLOCKWISE`.
    /// - No depth clamp.
    /// - No depth bias.
    pub fn fill_and_cull_anticlockwise() -> Self {
        Self {
            rasterizer_discard_enable: false,
            polygon_mode: vk::PolygonMode::FILL,
            line_width: 1.0,
            cull_mode: vk::CullModeFlags::BACK,
            front_face: vk::FrontFace::CLOCKWISE,
            depth_clamp_enable: false,
            depth_bias_enable: false,
            depth_bias_constant_factor: Default::default(),
            depth_bias_slope_factor: Default::default(),
            depth_bias_clamp: Default::default(),
        }
    }
}
