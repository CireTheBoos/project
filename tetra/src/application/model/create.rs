use ash::vk;
use vk_mem::Allocator;

use crate::context::{Device, device::QueueRoleFlags};

use super::{Clouds, Model, Shapes, Surfaces};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////////
// Function
/////////////////////////////////////////////////////////////////////////////

pub fn create(device: &Device, allocator: &Allocator) -> Result<Model> {
    // resources
    let clouds = Clouds::create(device, allocator)?;
    let surfaces = Surfaces::create(device, allocator)?;
    let shapes = Shapes::create(device, allocator)?;

    // command pool
    let create_info = vk::CommandPoolCreateInfo::default()
        .queue_family_index(device.queue_family_indices(QueueRoleFlags::TRANSFER)[0]);
    let transfer_command_pool = unsafe { device.create_command_pool(&create_info, None) }?;

    // command buffers
    let allocate_info = vk::CommandBufferAllocateInfo::default()
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_pool(transfer_command_pool)
        .command_buffer_count(1);
    let command_buffers = unsafe { device.allocate_command_buffers(&allocate_info) }?;
    let add = command_buffers[0];

    // sync
    let ready_to_add = device.create_fence_short(true)?;

    // create
    let mut model = Model {
        // resources
        clouds,
        surfaces,
        shapes,

        // commands
        transfer_command_pool,
        add,

        // sync
        ready_to_add,
    };

    init(device, &mut model)?;

    Ok(model)
}

/////////////////////////////////////////////////////////////////////////////
// Subfunctions
/////////////////////////////////////////////////////////////////////////////

/// - Zero diic buffer.
fn init(device: &Device, model: &mut Model) -> Result<()> {
    // command pool
    let create_info = vk::CommandPoolCreateInfo::default()
        .queue_family_index(device.queue_family_indices(QueueRoleFlags::TRANSFER)[0])
        .flags(vk::CommandPoolCreateFlags::TRANSIENT);
    let command_pool = unsafe { device.create_command_pool(&create_info, None) }?;

    // command buffers
    let allocate_info = vk::CommandBufferAllocateInfo::default()
        .command_pool(command_pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(1);
    let command_buffers = unsafe { device.allocate_command_buffers(&allocate_info) }?;
    let init = command_buffers[0];

    // begin `init`
    let begin_info =
        vk::CommandBufferBeginInfo::default().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
    unsafe { device.begin_command_buffer(init, &begin_info) }?;

    // fill diic buffer with 0 -> set instance count to 0 and ignore indirect draw parameters by default
    unsafe { device.cmd_fill_buffer(init, model.shapes.diic.buffer, 0, vk::WHOLE_SIZE, 0) };

    // end `init`
    unsafe { device.end_command_buffer(init) }?;

    // create fence to wait work
    let wait_until_finish = device.create_fence_short(false)?;

    // submit `init` : signal `ready_to_add` when done
    let command_buffer_info = vk::CommandBufferSubmitInfo::default().command_buffer(init);
    let command_buffer_infos = [command_buffer_info];
    let submit = vk::SubmitInfo2::default().command_buffer_infos(&command_buffer_infos);
    let submits = [submit];
    unsafe {
        device.queue_submit2(
            device.queue(QueueRoleFlags::TRANSFER).vk_queue,
            &submits,
            wait_until_finish,
        )
    }?;

    // wait fence
    let fences = [wait_until_finish];
    unsafe { device.wait_for_fences(&fences, true, u64::MAX) }?;

    // destroy all
    unsafe { device.destroy_command_pool(command_pool, None) };
    unsafe { device.destroy_fence(wait_until_finish, None) };

    Ok(())
}
