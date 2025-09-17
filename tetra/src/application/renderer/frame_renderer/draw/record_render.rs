use ash::vk;

use crate::context::{
    Device,
    device::{Execution, Memory, Transition, TransitionData, TransitionSync},
};

use super::{
    Depth, Swapchain,
    model::{self, Model},
    pipelines::RenderingPipeline,
};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////
// Function
/////////////////////////////////////////////////////////////////////////

pub fn record_render(
    device: &Device,
    model: &Model,
    render: vk::CommandBuffer,
    swapchain: &Swapchain,
    swapchain_image_index: u32,
    depth: &Depth,
    rendering_pipeline: &RenderingPipeline,
) -> Result<()> {
    // extract
    let swapchain_image = swapchain.images[swapchain_image_index as usize];
    let swapchain_image_view = swapchain.image_views[swapchain_image_index as usize];

    // begin
    let begin_info = vk::CommandBufferBeginInfo::default();
    unsafe { device.begin_command_buffer(render, &begin_info) }?;

    // transition swapchain image : UNDEFINED -> COLOR_ATTACHMENT_OPTIMAL
    device.cmd_transition_image(
        render,
        TransitionData {
            image: swapchain_image,
            subresource_range: swapchain.subresource_range,
        },
        TransitionSync {
            execution: Execution {
                previous_stages: vk::PipelineStageFlags2::NONE,
                next_stages: vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
            },
            memory: Memory {
                writes_availability: vk::AccessFlags2::NONE,
                reads_visibility: vk::AccessFlags2::NONE,
            },
        },
        Transition {
            old_layout: vk::ImageLayout::UNDEFINED,
            new_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        },
    );

    // transition depth image : UNDEFINED -> DEPTH_ATTACHMENT_OPTIMAL
    device.cmd_transition_image(
        render,
        TransitionData {
            image: depth.image,
            subresource_range: depth.subresource_range,
        },
        TransitionSync {
            execution: Execution {
                previous_stages: vk::PipelineStageFlags2::NONE,
                next_stages: vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS,
            },
            memory: Memory {
                writes_availability: vk::AccessFlags2::NONE,
                reads_visibility: vk::AccessFlags2::NONE,
            },
        },
        Transition {
            old_layout: vk::ImageLayout::UNDEFINED,
            new_layout: vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL,
        },
    );

    // bind pipeline
    unsafe {
        device.cmd_bind_pipeline(
            render,
            vk::PipelineBindPoint::GRAPHICS,
            **rendering_pipeline,
        )
    };

    // bind vertex buffer
    let buffers = [model.clouds.visible.buffer, model.shapes.info.buffer];
    let offsets = [0, 0];
    unsafe { device.cmd_bind_vertex_buffers2(render, 0, &buffers, &offsets, None, None) };

    // bind index buffer
    let index_type = model::INDEX_TYPE;
    unsafe { device.cmd_bind_index_buffer(render, model.surfaces.visible.buffer, 0, index_type) };

    // set scissor
    let scissors = [vk::Rect2D {
        offset: vk::Offset2D { x: 0, y: 0 },
        extent: swapchain.extent,
    }];
    unsafe { device.cmd_set_scissor(render, 0, &scissors) };

    // set viewport
    let viewports = [vk::Viewport::default()
        .x(0.)
        .y(swapchain.extent.height as f32)
        .width(swapchain.extent.width as f32)
        .height(-(swapchain.extent.height as f32))
        .min_depth(0.)
        .max_depth(1.)];
    unsafe { device.cmd_set_viewport(render, 0, &viewports) };

    //------// begin rendering //------//

    // swapchain attachment
    let black_clear_value = vk::ClearValue {
        color: vk::ClearColorValue {
            float32: [0., 0., 0., 0.],
        },
    };
    let swapchain_attachment = vk::RenderingAttachmentInfo::default()
        .image_view(swapchain_image_view)
        .image_layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .clear_value(black_clear_value);

    // depth attachment
    let far_clear_value = vk::ClearValue {
        depth_stencil: vk::ClearDepthStencilValue {
            depth: 1.,
            stencil: Default::default(),
        },
    };
    let depth_attachment = vk::RenderingAttachmentInfo::default()
        .image_view(depth.image_view)
        .image_layout(vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL)
        .load_op(vk::AttachmentLoadOp::CLEAR)
        .store_op(vk::AttachmentStoreOp::STORE)
        .clear_value(far_clear_value);

    // rendering info
    let color_attachments = [swapchain_attachment];
    let rendering_info = vk::RenderingInfo::default()
        .render_area(
            vk::Rect2D::default()
                .offset(vk::Offset2D { x: 0, y: 0 })
                .extent(swapchain.extent),
        )
        .color_attachments(&color_attachments)
        .depth_attachment(&depth_attachment)
        .layer_count(1);

    // cmd
    unsafe { device.cmd_begin_rendering(render, &rendering_info) };

    //------//

    // bind sets
    let descriptor_sets = [rendering_pipeline.scene_set];
    unsafe {
        device.cmd_bind_descriptor_sets(
            render,
            vk::PipelineBindPoint::GRAPHICS,
            rendering_pipeline.layout,
            0,
            &descriptor_sets,
            &[],
        )
    };

    // draw
    unsafe {
        device.cmd_draw_indexed_indirect(
            render,
            model.shapes.diic.buffer,
            0,
            model::MAX_SHAPES as u32,
            size_of::<vk::DrawIndexedIndirectCommand>() as u32,
        )
    };

    // end rendering
    unsafe { device.cmd_end_rendering(render) };

    // transition swapchain image : COLOR_ATTACHMENT_OPTIMAL -> PRESENT_SRC_KHR
    device.cmd_transition_image(
        render,
        TransitionData {
            image: swapchain_image,
            subresource_range: swapchain.subresource_range,
        },
        TransitionSync {
            execution: Execution {
                previous_stages: vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
                next_stages: vk::PipelineStageFlags2::BOTTOM_OF_PIPE,
            },
            memory: Memory {
                writes_availability: vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
                reads_visibility: vk::AccessFlags2::MEMORY_READ,
            },
        },
        Transition {
            old_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            new_layout: vk::ImageLayout::PRESENT_SRC_KHR,
        },
    );

    // end
    unsafe { device.end_command_buffer(render) }?;

    Ok(())
}
