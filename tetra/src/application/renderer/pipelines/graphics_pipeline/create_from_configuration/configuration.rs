pub mod states;

use ash::vk;

pub use states::{
    AttributeDescription, BindingConfiguration, ColorBlendConfiguration, DepthStencilConfiguration,
    InputAssemblyConfiguration, MultisampleConfiguration, RasterizationConfiguration,
    StatesConfiguration, VertexInputConfiguration, ViewportConfiguration,
};

/////////////////////////////////////////////////////////////////////////
// Structures
/////////////////////////////////////////////////////////////////////////

pub struct GraphicsPipelineConfiguration {
    pub states: states::StatesConfiguration,
    pub dynamic_states: Vec<vk::DynamicState>,
    pub stages: Vec<StageConfiguration>,
    pub layout: vk::PipelineLayout,

    // push next
    pub dynamic_rendering: DynamicRenderingConfiguration,
}

pub struct StageConfiguration {
    pub path: &'static str,
    pub stage: vk::ShaderStageFlags,
}

pub struct DynamicRenderingConfiguration {
    pub color_attachment_formats: Vec<vk::Format>,
    pub depth_attachment_format: vk::Format,
    pub stencil_attachment_format: vk::Format,
}

impl DynamicRenderingConfiguration {
    pub fn one_color_one_depth(
        color_format: vk::Format,
        depth_format: vk::Format,
    ) -> DynamicRenderingConfiguration {
        DynamicRenderingConfiguration {
            color_attachment_formats: vec![color_format],
            depth_attachment_format: depth_format,
            stencil_attachment_format: vk::Format::UNDEFINED,
        }
    }
}
