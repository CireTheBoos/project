use ash::vk;

/////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////

/// - Own data.
/// - Provide `contains_..` functions.
///
/// ```rust
/// // assemble using `vk::PhysicalDeviceFeatures2`
/// let features2 = vk::PhysicalDeviceFeatures2::default()
///    .features(features.vulkan10) // warning : not a &mut
///    .push_next(&mut features.vulkan11)
///    .push_next(&mut features.vulkan12)
///    .push_next(&mut features.vulkan13);
/// ```
#[derive(Debug, Default)]
pub struct Features {
    pub features_10: vk::PhysicalDeviceFeatures,
    // static lifetimes because `push_next` is not used
    pub features_11: vk::PhysicalDeviceVulkan11Features<'static>,
    pub features_12: vk::PhysicalDeviceVulkan12Features<'static>,
    pub features_13: vk::PhysicalDeviceVulkan13Features<'static>,
}

/////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////

/// Contains
impl Features {
    pub fn contains(&self, other: &Self) -> bool {
        self.contains_features_10(&other.features_10)
            && self.contains_features_11(&other.features_11)
            && self.contains_features_12(&other.features_12)
            && self.contains_features_13(&other.features_13)
    }

    pub fn contains_features_10(&self, features_10: &vk::PhysicalDeviceFeatures) -> bool {
        macro_rules! supported_or_not_required {
            ($field:ident) => {
                features_10.$field <= self.features_10.$field
            };
        }

        supported_or_not_required!(robust_buffer_access)
            && supported_or_not_required!(full_draw_index_uint32)
            && supported_or_not_required!(image_cube_array)
            && supported_or_not_required!(independent_blend)
            && supported_or_not_required!(geometry_shader)
            && supported_or_not_required!(tessellation_shader)
            && supported_or_not_required!(sample_rate_shading)
            && supported_or_not_required!(inherited_queries)
            && supported_or_not_required!(variable_multisample_rate)
            && supported_or_not_required!(sparse_residency_aliased)
            && supported_or_not_required!(sparse_residency16_samples)
            && supported_or_not_required!(sparse_residency8_samples)
            && supported_or_not_required!(sparse_residency4_samples)
            && supported_or_not_required!(sparse_residency2_samples)
            && supported_or_not_required!(sparse_residency_image3_d)
            && supported_or_not_required!(sparse_residency_image2_d)
            && supported_or_not_required!(sparse_residency_buffer)
            && supported_or_not_required!(sparse_binding)
            && supported_or_not_required!(shader_resource_min_lod)
            && supported_or_not_required!(shader_resource_residency)
            && supported_or_not_required!(shader_int16)
            && supported_or_not_required!(shader_int64)
            && supported_or_not_required!(shader_float64)
            && supported_or_not_required!(shader_cull_distance)
            && supported_or_not_required!(shader_clip_distance)
            && supported_or_not_required!(shader_storage_image_array_dynamic_indexing)
            && supported_or_not_required!(shader_storage_buffer_array_dynamic_indexing)
            && supported_or_not_required!(shader_sampled_image_array_dynamic_indexing)
            && supported_or_not_required!(shader_uniform_buffer_array_dynamic_indexing)
            && supported_or_not_required!(shader_storage_image_write_without_format)
            && supported_or_not_required!(shader_storage_image_read_without_format)
            && supported_or_not_required!(shader_storage_image_extended_formats)
            && supported_or_not_required!(shader_storage_image_multisample)
            && supported_or_not_required!(shader_image_gather_extended)
            && supported_or_not_required!(shader_tessellation_and_geometry_point_size)
            && supported_or_not_required!(fragment_stores_and_atomics)
            && supported_or_not_required!(vertex_pipeline_stores_and_atomics)
            && supported_or_not_required!(pipeline_statistics_query)
            && supported_or_not_required!(occlusion_query_precise)
            && supported_or_not_required!(texture_compression_bc)
            && supported_or_not_required!(texture_compression_astc_ldr)
            && supported_or_not_required!(texture_compression_etc2)
            && supported_or_not_required!(sampler_anisotropy)
            && supported_or_not_required!(multi_viewport)
            && supported_or_not_required!(alpha_to_one)
            && supported_or_not_required!(large_points)
            && supported_or_not_required!(wide_lines)
            && supported_or_not_required!(depth_bounds)
            && supported_or_not_required!(fill_mode_non_solid)
            && supported_or_not_required!(depth_bias_clamp)
            && supported_or_not_required!(depth_clamp)
            && supported_or_not_required!(draw_indirect_first_instance)
            && supported_or_not_required!(multi_draw_indirect)
            && supported_or_not_required!(logic_op)
            && supported_or_not_required!(dual_src_blend)
    }

    pub fn contains_features_11(&self, features_11: &vk::PhysicalDeviceVulkan11Features) -> bool {
        macro_rules! supported_or_not_required {
            ($field:ident) => {
                features_11.$field <= self.features_11.$field
            };
        }

        supported_or_not_required!(storage_buffer16_bit_access)
            && supported_or_not_required!(uniform_and_storage_buffer16_bit_access)
            && supported_or_not_required!(storage_push_constant16)
            && supported_or_not_required!(storage_input_output16)
            && supported_or_not_required!(multiview)
            && supported_or_not_required!(multiview_geometry_shader)
            && supported_or_not_required!(multiview_tessellation_shader)
            && supported_or_not_required!(variable_pointers_storage_buffer)
            && supported_or_not_required!(variable_pointers)
            && supported_or_not_required!(protected_memory)
            && supported_or_not_required!(sampler_ycbcr_conversion)
            && supported_or_not_required!(shader_draw_parameters)
    }

    pub fn contains_features_12(&self, features_12: &vk::PhysicalDeviceVulkan12Features) -> bool {
        macro_rules! supported_or_not_required {
            ($field:ident) => {
                features_12.$field <= self.features_12.$field
            };
        }

        supported_or_not_required!(subgroup_broadcast_dynamic_id)
            && supported_or_not_required!(shader_output_layer)
            && supported_or_not_required!(shader_output_viewport_index)
            && supported_or_not_required!(vulkan_memory_model_availability_visibility_chains)
            && supported_or_not_required!(vulkan_memory_model_device_scope)
            && supported_or_not_required!(vulkan_memory_model)
            && supported_or_not_required!(buffer_device_address_multi_device)
            && supported_or_not_required!(buffer_device_address_capture_replay)
            && supported_or_not_required!(buffer_device_address)
            && supported_or_not_required!(timeline_semaphore)
            && supported_or_not_required!(host_query_reset)
            && supported_or_not_required!(separate_depth_stencil_layouts)
            && supported_or_not_required!(shader_subgroup_extended_types)
            && supported_or_not_required!(uniform_buffer_standard_layout)
            && supported_or_not_required!(imageless_framebuffer)
            && supported_or_not_required!(scalar_block_layout)
            && supported_or_not_required!(sampler_filter_minmax)
            && supported_or_not_required!(runtime_descriptor_array)
            && supported_or_not_required!(descriptor_binding_variable_descriptor_count)
            && supported_or_not_required!(descriptor_binding_partially_bound)
            && supported_or_not_required!(descriptor_binding_update_unused_while_pending)
            && supported_or_not_required!(descriptor_binding_storage_texel_buffer_update_after_bind)
            && supported_or_not_required!(descriptor_binding_uniform_texel_buffer_update_after_bind)
            && supported_or_not_required!(descriptor_binding_storage_buffer_update_after_bind)
            && supported_or_not_required!(descriptor_binding_storage_image_update_after_bind)
            && supported_or_not_required!(descriptor_binding_sampled_image_update_after_bind)
            && supported_or_not_required!(descriptor_binding_uniform_buffer_update_after_bind)
            && supported_or_not_required!(shader_storage_texel_buffer_array_non_uniform_indexing)
            && supported_or_not_required!(shader_uniform_texel_buffer_array_non_uniform_indexing)
            && supported_or_not_required!(shader_input_attachment_array_non_uniform_indexing)
            && supported_or_not_required!(shader_storage_image_array_non_uniform_indexing)
            && supported_or_not_required!(shader_storage_buffer_array_non_uniform_indexing)
            && supported_or_not_required!(shader_sampled_image_array_non_uniform_indexing)
            && supported_or_not_required!(shader_uniform_buffer_array_non_uniform_indexing)
            && supported_or_not_required!(shader_storage_texel_buffer_array_dynamic_indexing)
            && supported_or_not_required!(shader_uniform_texel_buffer_array_dynamic_indexing)
            && supported_or_not_required!(shader_input_attachment_array_dynamic_indexing)
            && supported_or_not_required!(descriptor_indexing)
            && supported_or_not_required!(shader_int8)
            && supported_or_not_required!(shader_float16)
            && supported_or_not_required!(shader_shared_int64_atomics)
            && supported_or_not_required!(shader_buffer_int64_atomics)
            && supported_or_not_required!(storage_push_constant8)
            && supported_or_not_required!(uniform_and_storage_buffer8_bit_access)
            && supported_or_not_required!(storage_buffer8_bit_access)
            && supported_or_not_required!(draw_indirect_count)
            && supported_or_not_required!(sampler_mirror_clamp_to_edge)
    }

    pub fn contains_features_13(&self, features_13: &vk::PhysicalDeviceVulkan13Features) -> bool {
        macro_rules! supported_or_not_required {
            ($field:ident) => {
                features_13.$field <= self.features_13.$field
            };
        }

        supported_or_not_required!(robust_image_access)
            && supported_or_not_required!(inline_uniform_block)
            && supported_or_not_required!(descriptor_binding_inline_uniform_block_update_after_bind)
            && supported_or_not_required!(pipeline_creation_cache_control)
            && supported_or_not_required!(private_data)
            && supported_or_not_required!(shader_demote_to_helper_invocation)
            && supported_or_not_required!(shader_terminate_invocation)
            && supported_or_not_required!(subgroup_size_control)
            && supported_or_not_required!(compute_full_subgroups)
            && supported_or_not_required!(synchronization2)
            && supported_or_not_required!(texture_compression_astc_hdr)
            && supported_or_not_required!(shader_zero_initialize_workgroup_memory)
            && supported_or_not_required!(dynamic_rendering)
            && supported_or_not_required!(shader_integer_dot_product)
            && supported_or_not_required!(maintenance4)
    }
}
