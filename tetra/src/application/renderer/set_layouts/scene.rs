use std::ops::Deref;

use ash::vk;

use crate::context::Device;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct Scene {
    layout: vk::DescriptorSetLayout,

    // bindings
    pub camera: vk::DescriptorSetLayoutBinding<'static>, // lifetime -> for immutable samplers pointers
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Deref : `vk::DescriptorSetLayout`
impl Deref for Scene {
    type Target = vk::DescriptorSetLayout;

    fn deref(&self) -> &Self::Target {
        &self.layout
    }
}

/// Create & Destroy
impl Scene {
    pub fn create(device: &Device) -> Result<Self> {
        //------// bindings //------//

        let camera = vk::DescriptorSetLayoutBinding::default()
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::VERTEX)
            .binding(0);

        //------// layout //------//

        let bindings = [camera];
        let create_info = vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings);
        let layout = unsafe { device.create_descriptor_set_layout(&create_info, None) }?;

        //------//

        Ok(Self { camera, layout })
    }

    pub fn destroy(&mut self, device: &Device) {
        unsafe { device.destroy_descriptor_set_layout(self.layout, None) };
    }
}

/// Set layout
impl Scene {
    pub fn pool_sizes(&self) -> Vec<vk::DescriptorPoolSize> {
        // uniform buffer
        let uniform_buffer = vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(self.camera.descriptor_count);

        vec![uniform_buffer]
    }

    pub fn update_set(
        &self,
        device: &Device,
        set: vk::DescriptorSet,
        camera_buffer_info: vk::DescriptorBufferInfo,
    ) {
        //------// writes //------//

        // camera
        let camera_buffer_info = [camera_buffer_info];
        let camera_write = vk::WriteDescriptorSet::default()
            .dst_set(set)
            .dst_binding(self.camera.binding)
            .descriptor_type(self.camera.descriptor_type)
            .dst_array_element(0)
            .descriptor_count(self.camera.descriptor_count)
            .buffer_info(&camera_buffer_info);

        let descriptor_writes = [camera_write];

        //------//

        unsafe { device.update_descriptor_sets(&descriptor_writes, &[]) };
    }
}
