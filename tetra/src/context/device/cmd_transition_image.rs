use ash::vk;

use super::Device;

/////////////////////////////////////////////////////////////////////////
// Argument
/////////////////////////////////////////////////////////////////////////

pub struct TransitionData {
    pub image: vk::Image,
    pub subresource_range: vk::ImageSubresourceRange,
}

pub struct TransitionSync {
    pub execution: Execution,
    pub memory: Memory,
}
pub struct Execution {
    pub previous_stages: vk::PipelineStageFlags2,
    pub next_stages: vk::PipelineStageFlags2,
}
pub struct Memory {
    pub writes_availability: vk::AccessFlags2,
    pub reads_visibility: vk::AccessFlags2,
}

pub struct Transition {
    pub old_layout: vk::ImageLayout,
    pub new_layout: vk::ImageLayout,
}

/////////////////////////////////////////////////////////////////////////
// Function
/////////////////////////////////////////////////////////////////////////

pub fn cmd_transition_image(
    device: &Device,
    command_buffer: vk::CommandBuffer,
    data: TransitionData,
    sync: TransitionSync,
    transition: Transition,
) {
    let image_memory_barrier = vk::ImageMemoryBarrier2::default()
        // data
        .image(data.image)
        .subresource_range(data.subresource_range)
        // sync
        .src_stage_mask(sync.execution.previous_stages)
        .src_access_mask(sync.memory.writes_availability)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_stage_mask(sync.execution.next_stages)
        .dst_access_mask(sync.memory.reads_visibility)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        // transition
        .old_layout(transition.old_layout)
        .new_layout(transition.new_layout);

    let image_memory_barriers = [image_memory_barrier];

    let dependency_info =
        vk::DependencyInfo::default().image_memory_barriers(&image_memory_barriers);

    unsafe { device.cmd_pipeline_barrier2(command_buffer, &dependency_info) };
}
