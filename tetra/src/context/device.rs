mod cmd_transition_image;
mod create;
mod queues;
mod sync;

use std::{ffi::CString, ops::Deref};

use ash::{prelude::VkResult, vk};

use super::Instance;

pub use cmd_transition_image::{Execution, Memory, Transition, TransitionData, TransitionSync};
pub use queues::{Queue, QueueRoleFlags};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////

/// - Wrap loaders.
/// - Manage queues using `QueueRoleFlags`:
///     - `queue(role)`
///     - `queue_families(roles)`
/// - Provide additional functions.
pub struct Device {
    // loaders
    device: ash::Device,
    pub swapchain_device: ash::khr::swapchain::Device,
    pub debug_utils_device: ash::ext::debug_utils::Device,

    // queues
    queues: Vec<Queue>,

    // arguments
    pub physical_device: vk::PhysicalDevice,
    pub surface: vk::SurfaceKHR,
}

/////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////

/// Deref : `ash::Device`
impl Deref for Device {
    type Target = ash::Device;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

/// Create & Destroy
impl Device {
    pub fn create(instance: &Instance, surface: vk::SurfaceKHR) -> Result<Self> {
        create::create(instance, surface)
    }

    pub fn destroy(&mut self) {
        unsafe { self.destroy_device(None) };
    }
}

/// Queues
impl Device {
    pub fn queue(&self, role: QueueRoleFlags) -> Queue {
        queues::queue(&self.queues, role)
    }

    pub fn queue_family_indices(&self, roles: QueueRoleFlags) -> Vec<u32> {
        queues::queue_family_indices(&self.queues, roles)
    }
}

/// Debug
impl Device {
    pub fn name(
        &self,
        object_handle: impl vk::Handle,
        object_name: CString,
    ) -> ash::prelude::VkResult<()> {
        let name_info = vk::DebugUtilsObjectNameInfoEXT::default()
            .object_handle(object_handle)
            .object_name(&object_name);

        unsafe {
            Ok(self
                .debug_utils_device
                .set_debug_utils_object_name(&name_info)?)
        }
    }
}

/// Sync
impl Device {
    #[allow(unused)]
    pub fn create_timeline_short(&self, initial_value: u64) -> VkResult<vk::Semaphore> {
        sync::create_timeline_short(self, initial_value)
    }

    pub fn create_binary_short(&self) -> ash::prelude::VkResult<vk::Semaphore> {
        sync::create_binary_short(self)
    }

    pub fn create_fence_short(&self, already_signaled: bool) -> ash::prelude::VkResult<vk::Fence> {
        sync::create_fence_short(self, already_signaled)
    }

    pub fn queue_present_single_swapchain(
        &self,
        swapchain: vk::SwapchainKHR,
        swapchain_image_index: u32,
        wait_binary_semaphores: &[vk::Semaphore],
    ) -> VkResult<bool> {
        sync::queue_present_single_swapchain(
            self,
            swapchain,
            swapchain_image_index,
            wait_binary_semaphores,
        )
    }
}

/// Commands
impl Device {
    pub fn cmd_transition_image(
        &self,
        command_buffer: vk::CommandBuffer,
        data: TransitionData,
        sync: TransitionSync,
        transition: Transition,
    ) {
        cmd_transition_image::cmd_transition_image(self, command_buffer, data, sync, transition);
    }
}
