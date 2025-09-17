use ash::vk;
use vk_mem::Alloc;

use crate::context::{Device, Instance, device::QueueRoleFlags};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

const DEPTH_FORMAT: vk::Format = vk::Format::D32_SFLOAT;
const DEPTH_USAGE: vk::FormatFeatureFlags2 = vk::FormatFeatureFlags2::DEPTH_STENCIL_ATTACHMENT;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct Depth {
    pub allocation: vk_mem::Allocation,
    pub image: vk::Image,
    pub subresource_range: vk::ImageSubresourceRange,
    pub image_view: vk::ImageView,
    pub format: vk::Format,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Create per field & Destroy
impl Depth {
    pub fn create(
        instance: &Instance,
        device: &Device,
        allocator: &vk_mem::Allocator,
        swapchain_extent: vk::Extent2D,
    ) -> Result<Depth> {
        // check
        instance.physical_device_support_format_usage_in_optimal_tiling(
            device.physical_device,
            DEPTH_FORMAT,
            DEPTH_USAGE,
        )?;

        //------// image create info //------//

        let extent = vk::Extent3D {
            width: swapchain_extent.width,
            height: swapchain_extent.height,
            depth: 1,
        };
        let queue_family_indices = device.queue_family_indices(QueueRoleFlags::RENDER);
        let image_create_info = vk::ImageCreateInfo::default()
            // layout
            .tiling(vk::ImageTiling::OPTIMAL)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            // data
            .image_type(vk::ImageType::TYPE_2D)
            .extent(extent)
            .format(DEPTH_FORMAT)
            .array_layers(1)
            .mip_levels(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            // access
            .usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
            .queue_family_indices(&queue_family_indices)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        //------// allocation create info //------//

        let allocation_create_info = vk_mem::AllocationCreateInfo {
            usage: vk_mem::MemoryUsage::AutoPreferDevice,
            ..Default::default()
        };

        //------// create & allocate image //------//

        let (image, allocation) =
            unsafe { allocator.create_image(&image_create_info, &allocation_create_info) }?;

        //------// create image view //------//

        let image_view_create_info = vk::ImageViewCreateInfo::default()
            .image(image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(DEPTH_FORMAT)
            .subresource_range(
                vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::DEPTH)
                    .layer_count(1)
                    .level_count(1),
            )
            .components(vk::ComponentMapping::default());

        let image_view = unsafe { device.create_image_view(&image_view_create_info, None)? };

        //------// subresource range //------//

        let subresource_range = vk::ImageSubresourceRange::default()
            .aspect_mask(vk::ImageAspectFlags::DEPTH)
            .layer_count(1)
            .level_count(1);

        //------//

        Ok(Depth {
            allocation,
            image,
            subresource_range,
            image_view,
            format: DEPTH_FORMAT,
        })
    }

    pub fn destroy(&mut self, device: &Device, allocator: &vk_mem::Allocator) {
        unsafe {
            device.destroy_image_view(self.image_view, None);
            allocator.destroy_image(self.image, &mut self.allocation);
        }
    }
}
