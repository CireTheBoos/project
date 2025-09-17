use ash::vk;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct MultisampleConfiguration {
    // multisampling
    pub rasterization_samples: vk::SampleCountFlags,
    pub sample_mask: Vec<u32>,
    pub alpha_to_one_enable: bool,

    // sample shading
    pub sample_shading_enable: bool,
    pub min_sample_shading: f32,

    // ATOC
    pub alpha_to_coverage_enable: bool,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Constructors
impl MultisampleConfiguration {
    /// - Single sample per fragment (alpha allowed).
    /// - No sample shading.
    /// - No ATOC.
    pub fn no_multisampling() -> Self {
        Self {
            rasterization_samples: vk::SampleCountFlags::TYPE_1,
            sample_mask: Vec::new(),
            alpha_to_one_enable: false,
            sample_shading_enable: false,
            min_sample_shading: Default::default(),
            alpha_to_coverage_enable: false,
        }
    }
}
