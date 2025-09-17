use ash::vk;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct ColorBlendConfiguration {
    // classic blending
    pub attachments: Vec<vk::PipelineColorBlendAttachmentState>,
    pub blend_constants: [f32; 4],

    // bitwise blending
    pub logic_op_enable: bool,
    pub logic_op: vk::LogicOp,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Constructors
impl ColorBlendConfiguration {
    /// - Single attachment with RGBA color write mask.
    /// - No blending.
    pub fn no_blending() -> Self {
        let attachment: vk::PipelineColorBlendAttachmentState =
            vk::PipelineColorBlendAttachmentState::default()
                .color_write_mask(vk::ColorComponentFlags::RGBA)
                .blend_enable(false);

        Self {
            attachments: vec![attachment],
            blend_constants: Default::default(),
            logic_op_enable: false,
            logic_op: vk::LogicOp::default(),
        }
    }
}
