use ash::vk;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct ViewportConfiguration {
    pub scissors: Vec<vk::Rect2D>,
    pub scissor_count: u32,
    pub viewports: Vec<vk::Viewport>,
    pub viewport_count: u32,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Constructors
impl ViewportConfiguration {
    pub fn dynamic(scissor_count: u32, viewport_count: u32) -> ViewportConfiguration {
        ViewportConfiguration {
            scissors: Vec::new(),
            scissor_count,
            viewports: Vec::new(),
            viewport_count,
        }
    }
}
