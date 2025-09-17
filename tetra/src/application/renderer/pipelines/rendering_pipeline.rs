use std::{mem::offset_of, ops::Deref};

use ash::vk;

use crate::context::Device;

use super::{
    graphics_pipeline::{configuration::*, create_from_configuration},
    model::{ShapeInfo, Vertex},
    set_layouts::Scene as SceneSetLayout,
};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////

pub struct RenderingPipeline {
    pipeline: vk::Pipeline,
    pub layout: vk::PipelineLayout,

    // descriptors
    pub descriptor_pool: vk::DescriptorPool,
    pub scene_set: vk::DescriptorSet,
}

/////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////

/// Deref : `vk::Pipeline`
impl Deref for RenderingPipeline {
    type Target = vk::Pipeline;

    fn deref(&self) -> &Self::Target {
        &self.pipeline
    }
}

/// Create & Destroy
impl RenderingPipeline {
    pub fn create(
        device: &Device,
        scene_set_layout: &SceneSetLayout,
        swapchain_format: vk::Format,
        depth_format: vk::Format,
    ) -> Result<Self> {
        //------// descriptors //------//

        // pool
        let pool_sizes = scene_set_layout.pool_sizes();
        let create_info = vk::DescriptorPoolCreateInfo::default()
            .pool_sizes(&pool_sizes)
            .max_sets(1);
        let descriptor_pool = unsafe { device.create_descriptor_pool(&create_info, None) }?;

        // set layouts
        let set_layouts = vec![**scene_set_layout];

        // allocation
        let allocate_info = vk::DescriptorSetAllocateInfo::default()
            .descriptor_pool(descriptor_pool)
            .set_layouts(&set_layouts);
        let sets = unsafe { device.allocate_descriptor_sets(&allocate_info) }?;

        let scene_set = sets[0];

        //------// layout //------//

        // create info
        let layout_create_info = vk::PipelineLayoutCreateInfo::default()
            .set_layouts(&set_layouts)
            .push_constant_ranges(&[]);

        let layout = unsafe { device.create_pipeline_layout(&layout_create_info, None) }?;

        //------// pipeline //------//

        // vertex input bindings
        let vertex_input_bindings = vec![
            // vertices
            BindingConfiguration {
                binding: 0,
                stride: size_of::<Vertex>() as u32,
                input_rate: vk::VertexInputRate::VERTEX,
                attributes: vec![
                    // position
                    AttributeDescription {
                        location: 0,
                        format: vk::Format::R32G32B32_SFLOAT,
                        offset: offset_of!(Vertex, position_in_shape) as u32,
                    },
                ],
            },
            // instances
            BindingConfiguration {
                binding: 1,
                stride: size_of::<ShapeInfo>() as u32,
                input_rate: vk::VertexInputRate::INSTANCE,
                attributes: vec![
                    // position
                    AttributeDescription {
                        location: 1,
                        format: vk::Format::R32G32B32_SFLOAT,
                        offset: offset_of!(ShapeInfo, position) as u32,
                    },
                    // scale
                    AttributeDescription {
                        location: 2,
                        format: vk::Format::R32_SFLOAT,
                        offset: offset_of!(ShapeInfo, scale) as u32,
                    },
                    // orientation
                    AttributeDescription {
                        location: 3,
                        format: vk::Format::R32G32B32A32_SFLOAT,
                        offset: offset_of!(ShapeInfo, orientation) as u32,
                    },
                ],
            },
        ];

        // states configuration
        let states = StatesConfiguration {
            color_blend: ColorBlendConfiguration::no_blending(),
            depth_stencil: DepthStencilConfiguration::test_depth_less_and_overwrite_on_success(),
            input_assembly: InputAssemblyConfiguration::triangle_list(),
            multisample: MultisampleConfiguration::no_multisampling(),
            rasterization: RasterizationConfiguration::fill_and_cull_anticlockwise(),
            vertex_input: VertexInputConfiguration {
                bindings: vertex_input_bindings,
            },
            viewport: ViewportConfiguration::dynamic(1, 1),
        };

        // dynamic states configuration
        let dynamic_states = vec![vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];

        // stages configuration
        let stages = vec![
            StageConfiguration {
                path: "shaders/vertex.spv",
                stage: vk::ShaderStageFlags::VERTEX,
            },
            StageConfiguration {
                path: "shaders/geometry.spv",
                stage: vk::ShaderStageFlags::GEOMETRY,
            },
            StageConfiguration {
                path: "shaders/fragment.spv",
                stage: vk::ShaderStageFlags::FRAGMENT,
            },
        ];

        // dynamic rendering configuration
        let dynamic_rendering =
            DynamicRenderingConfiguration::one_color_one_depth(swapchain_format, depth_format);

        // configuration
        let pipeline_configuration = GraphicsPipelineConfiguration {
            states,
            dynamic_states,
            stages,
            layout,
            dynamic_rendering,
        };

        let pipeline = create_from_configuration(device, pipeline_configuration)?;

        //------//

        Ok(Self {
            pipeline,
            layout,
            descriptor_pool,
            scene_set,
        })
    }

    pub fn destroy(&mut self, device: &Device) {
        unsafe {
            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.layout, None);
            device.destroy_descriptor_pool(self.descriptor_pool, None);
        }
    }
}
