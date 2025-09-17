pub mod configuration;

use std::ffi::CStr;

use ash::vk;

use crate::context::Device;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

const SHADER_ENTRY: &CStr = c"main";

/////////////////////////////////////////////////////////////////////////
// Function
/////////////////////////////////////////////////////////////////////////

pub fn create_from_configuration(
    device: &Device,
    configuration: configuration::GraphicsPipelineConfiguration,
) -> Result<vk::Pipeline> {
    //---// States //---//

    // vertex input
    let vertex_input_conf = configuration.states.vertex_input;
    let vertex_binding_descriptions = vertex_input_conf.vertex_binding_descriptions();
    let vertex_attribute_descriptions = vertex_input_conf.vertex_attribute_descriptions();
    let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::default()
        .vertex_binding_descriptions(&vertex_binding_descriptions)
        .vertex_attribute_descriptions(&vertex_attribute_descriptions);

    // input assembly
    let input_assembly_conf = configuration.states.input_assembly;
    let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::default()
        .topology(input_assembly_conf.topology)
        .primitive_restart_enable(input_assembly_conf.primitive_restart_enable);

    // rasterization
    let rasterization_conf = configuration.states.rasterization;
    let rasterization_state = vk::PipelineRasterizationStateCreateInfo::default()
        .rasterizer_discard_enable(rasterization_conf.rasterizer_discard_enable)
        .polygon_mode(rasterization_conf.polygon_mode)
        .line_width(rasterization_conf.line_width)
        .cull_mode(rasterization_conf.cull_mode)
        .front_face(rasterization_conf.front_face)
        .depth_clamp_enable(rasterization_conf.depth_clamp_enable)
        .depth_bias_enable(rasterization_conf.depth_bias_enable)
        .depth_bias_constant_factor(rasterization_conf.depth_bias_constant_factor)
        .depth_bias_slope_factor(rasterization_conf.depth_bias_slope_factor)
        .depth_bias_clamp(rasterization_conf.depth_bias_clamp);

    // depth-stencil
    let depth_stencil_conf = configuration.states.depth_stencil;
    let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo::default()
        .depth_test_enable(depth_stencil_conf.depth_test_enable)
        .depth_compare_op(depth_stencil_conf.depth_compare_op)
        .depth_write_enable(depth_stencil_conf.depth_write_enable)
        .depth_bounds_test_enable(depth_stencil_conf.depth_bounds_test_enable)
        .min_depth_bounds(depth_stencil_conf.min_depth_bounds)
        .max_depth_bounds(depth_stencil_conf.max_depth_bounds)
        .stencil_test_enable(depth_stencil_conf.stencil_test_enable)
        .back(depth_stencil_conf.back)
        .front(depth_stencil_conf.front);

    // multisample
    let multisample_conf = configuration.states.multisample;
    let multisample_state = vk::PipelineMultisampleStateCreateInfo::default()
        .rasterization_samples(multisample_conf.rasterization_samples)
        .sample_mask(&multisample_conf.sample_mask)
        .alpha_to_one_enable(multisample_conf.alpha_to_one_enable)
        .sample_shading_enable(multisample_conf.sample_shading_enable)
        .min_sample_shading(multisample_conf.min_sample_shading)
        .alpha_to_coverage_enable(multisample_conf.alpha_to_coverage_enable);

    // color blend
    let color_blend_conf = configuration.states.color_blend;
    let color_blend_state = vk::PipelineColorBlendStateCreateInfo::default()
        .attachments(&color_blend_conf.attachments)
        .blend_constants(color_blend_conf.blend_constants)
        .logic_op_enable(color_blend_conf.logic_op_enable)
        .logic_op(color_blend_conf.logic_op);

    // viewport
    let viewport_conf = configuration.states.viewport;
    let viewport_state = vk::PipelineViewportStateCreateInfo::default()
        .viewports(&viewport_conf.viewports)
        .scissors(&viewport_conf.scissors)
        .scissor_count(viewport_conf.scissor_count)
        .viewport_count(viewport_conf.viewport_count);

    //---// Dynamic states //---//

    let dynamic_state =
        vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&configuration.dynamic_states);

    //---// Stages //---//

    let mut shader_modules = Vec::with_capacity(configuration.stages.len());
    let mut stages = Vec::with_capacity(configuration.stages.len());
    for stage_conf in configuration.stages {
        let module = create_shader_module(device, &stage_conf)?;
        let stage = vk::PipelineShaderStageCreateInfo::default()
            .module(module)
            .stage(stage_conf.stage)
            .name(SHADER_ENTRY);
        shader_modules.push(module);
        stages.push(stage);
    }

    //---// Push next : Dynamic rendering //---//

    let dynamic_rendering_conf = configuration.dynamic_rendering;
    let mut dynamic_rendering = vk::PipelineRenderingCreateInfo::default()
        .color_attachment_formats(&dynamic_rendering_conf.color_attachment_formats)
        .depth_attachment_format(dynamic_rendering_conf.depth_attachment_format)
        .stencil_attachment_format(dynamic_rendering_conf.stencil_attachment_format);

    //---// Create //---//

    let create_info = vk::GraphicsPipelineCreateInfo::default()
        // states
        .color_blend_state(&color_blend_state)
        .depth_stencil_state(&depth_stencil_state)
        .input_assembly_state(&input_assembly_state)
        .multisample_state(&multisample_state)
        .rasterization_state(&rasterization_state)
        .vertex_input_state(&vertex_input_state)
        .viewport_state(&viewport_state)
        // dynamic states
        .dynamic_state(&dynamic_state)
        // stages
        .stages(&stages)
        // layout
        .layout(configuration.layout)
        // dynamic rendering
        .push_next(&mut dynamic_rendering);

    let create_infos = vec![create_info];
    let graphics_pipelines = unsafe {
        device
            .create_graphics_pipelines(vk::PipelineCache::null(), &create_infos, None)
            .map_err(|(_, error)| error)?
    };

    for module in shader_modules {
        unsafe { device.destroy_shader_module(module, None) };
    }

    Ok(graphics_pipelines[0])
}

fn create_shader_module(
    device: &Device,
    configuration: &configuration::StageConfiguration,
) -> Result<vk::ShaderModule> {
    let mut spv_file = std::fs::File::open(configuration.path)?;
    let spv_code = ash::util::read_spv(&mut spv_file)?;
    let create_info = vk::ShaderModuleCreateInfo::default().code(&spv_code);
    unsafe { Ok(device.create_shader_module(&create_info, None)?) }
}
