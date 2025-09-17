mod color_blend;
mod depth_stencil;
mod input_assembly;
mod multisample;
mod rasterization;
mod vertex_input;
mod viewport;

pub use color_blend::ColorBlendConfiguration;
pub use depth_stencil::DepthStencilConfiguration;
pub use input_assembly::InputAssemblyConfiguration;
pub use multisample::MultisampleConfiguration;
pub use rasterization::RasterizationConfiguration;
pub use vertex_input::{AttributeDescription, BindingConfiguration, VertexInputConfiguration};
pub use viewport::ViewportConfiguration;

/////////////////////////////////////////////////////////////////////////////

pub struct StatesConfiguration {
    pub color_blend: ColorBlendConfiguration,
    pub depth_stencil: DepthStencilConfiguration,
    pub input_assembly: InputAssemblyConfiguration,
    pub multisample: MultisampleConfiguration,
    pub rasterization: RasterizationConfiguration,
    pub vertex_input: VertexInputConfiguration,
    pub viewport: ViewportConfiguration,
}
