use ash::vk;

/////////////////////////////////////////////////////////////////////////////
// Structures
/////////////////////////////////////////////////////////////////////////////

pub struct VertexInputConfiguration {
    pub bindings: Vec<BindingConfiguration>,
}

pub struct BindingConfiguration {
    pub binding: u32,
    pub stride: u32,
    pub input_rate: vk::VertexInputRate,
    pub attributes: Vec<AttributeDescription>,
}

pub struct AttributeDescription {
    pub location: u32,
    pub format: vk::Format,
    pub offset: u32,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Constructors
impl VertexInputConfiguration {
    /// - No bindings.
    pub fn no_input() -> VertexInputConfiguration {
        VertexInputConfiguration {
            bindings: Vec::new(),
        }
    }
}

/// Vulkan conversions
impl VertexInputConfiguration {
    pub fn vertex_binding_descriptions(&self) -> Vec<vk::VertexInputBindingDescription> {
        self.bindings
            .iter()
            .map(|binding_conf| vk::VertexInputBindingDescription {
                binding: binding_conf.binding,
                stride: binding_conf.stride,
                input_rate: binding_conf.input_rate,
            })
            .collect()
    }

    pub fn vertex_attribute_descriptions(&self) -> Vec<vk::VertexInputAttributeDescription> {
        let mut vertex_attribute_descriptions = Vec::with_capacity(
            self.bindings
                .iter()
                .fold(0, |attributes_total_len, binding| {
                    attributes_total_len + binding.attributes.len()
                }),
        );
        for binding_conf in &self.bindings {
            for attribute in &binding_conf.attributes {
                vertex_attribute_descriptions.push(vk::VertexInputAttributeDescription {
                    location: attribute.location,
                    binding: binding_conf.binding,
                    format: attribute.format,
                    offset: attribute.offset,
                });
            }
        }
        vertex_attribute_descriptions
    }
}
