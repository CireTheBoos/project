use std::mem::offset_of;

use ash::vk;
use glam::{Quat, Vec3};
use mem_utils::{IndexOf, RangeOf};
use suballocation::{ArrayOfUnitSuballocation, UnitSuballocation};
use vk_mem::Allocator;

use crate::context::{Device, device::QueueRoleFlags};

use super::{Model, Shape, ShapeInfo, Triangle, Vertex, clouds, surfaces};

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////////
// Argument
/////////////////////////////////////////////////////////////////////////////

pub struct ShapeData {
    pub cloud: Vec<Vertex>,
    pub visible_cloud_len: usize,
    pub surface: Vec<Triangle>,
    pub visible_surface_len: usize,
    pub position: Vec3,
    pub scale: f32,
    pub orientation: Quat,
}

/////////////////////////////////////////////////////////////////////////////
// Function
/////////////////////////////////////////////////////////////////////////////

pub fn add(
    model: &mut Model,
    device: &Device,
    allocator: &Allocator,
    shape: ShapeData,
) -> Result<()> {
    // check input data
    check_shape(&shape)?;

    // allocate
    let allocations = allocate(model, &shape)?;

    // write (CPU-side)
    write(model, allocator, allocations, shape)?;

    // upload (GPU-side)
    upload(model, device, allocations)?;

    //////
    Ok(())
}

/////////////////////////////////////////////////////////////////////////////
// Subfonctions
/////////////////////////////////////////////////////////////////////////////

fn check_shape(shape: &ShapeData) -> Result<()> {
    // check not too many : vertices / visible vertices / triangles / visible triangles
    if shape.cloud.len() > clouds::MAX_VERTICES_PER_FULL_CLOUD {
        return Err("too much vertices".into());
    }
    if shape.visible_cloud_len > clouds::MAX_VERTICES_PER_VISIBLE_CLOUD {
        return Err("too much visible vertices".into());
    }
    if shape.surface.len() > surfaces::MAX_TRIANGLES_PER_FULL_SURFACE {
        return Err("too much triangles".into());
    }
    if shape.visible_surface_len > surfaces::MAX_TRIANGLES_PER_VISIBLE_SURFACE {
        return Err("too much visible triangles".into());
    }

    //////
    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Allocations {
    cloud: RangeOf<Vertex>,
    visible_cloud: RangeOf<Vertex>,
    surface: RangeOf<Triangle>,
    visible_surface: RangeOf<Triangle>,

    // same index underneath
    shape: IndexOf<Shape>,
    shape_info: IndexOf<ShapeInfo>,
    shape_diic: IndexOf<vk::DrawIndexedIndirectCommand>,
}

fn allocate(model: &mut Model, shape: &ShapeData) -> Result<Allocations> {
    // check if enough memory
    if !model
        .clouds
        .full
        .suballocator
        .can_allocate(shape.cloud.len())
        || !model
            .clouds
            .visible
            .suballocator
            .can_allocate(shape.visible_cloud_len)
        || !model
            .surfaces
            .full
            .suballocator
            .can_allocate(shape.surface.len())
        || !model
            .surfaces
            .visible
            .suballocator
            .can_allocate(shape.visible_surface_len)
        || !model.shapes.full.suballocator.can_allocate()
    {
        return Err("not enough memory".into());
    }

    // allocate
    let cloud = model.clouds.full.suballocator.allocate(shape.cloud.len())?;
    let visible_cloud = model
        .clouds
        .visible
        .suballocator
        .allocate(shape.visible_cloud_len)?;
    let surface = model
        .surfaces
        .full
        .suballocator
        .allocate(shape.surface.len())?;
    let visible_surface = model
        .surfaces
        .visible
        .suballocator
        .allocate(shape.visible_surface_len)?;
    let shape = model.shapes.full.suballocator.allocate()?;

    Ok(Allocations {
        cloud,
        visible_cloud,
        surface,
        visible_surface,
        shape,
        shape_info: IndexOf::new(shape.index),
        shape_diic: IndexOf::new(shape.index),
    })
}

fn write(
    model: &mut Model,
    allocator: &Allocator,
    allocations: Allocations,
    shape: ShapeData,
) -> Result<()> {
    // cloud
    model.clouds.full.memory[allocations.cloud.to_std_range()]
        .copy_from_slice(shape.cloud.as_slice());

    // surface
    model.surfaces.full.memory[allocations.surface.to_std_range()]
        .copy_from_slice(shape.surface.as_slice());

    // shape
    model.shapes.full.memory[allocations.shape.index] = Shape {
        // cloud
        cloud: allocations.cloud,
        visible_cloud: allocations.visible_cloud,

        // surface
        surface: allocations.surface,
        visible_surface: allocations.visible_surface,

        // shape data
        info: ShapeInfo {
            position: shape.position,
            scale: shape.scale,
            orientation: shape.orientation,
        },
        diic: vk::DrawIndexedIndirectCommand {
            index_count: (allocations.visible_surface.size * 3) as u32,
            instance_count: 1,
            first_index: (allocations.visible_surface.offset * 3) as u32,
            vertex_offset: allocations.visible_cloud.offset as i32,
            first_instance: allocations.shape_info.index as u32,
        },
    };

    // flush
    let flush_allocations = [
        &model.clouds.full.allocation,
        &model.surfaces.full.allocation,
        &model.shapes.full.allocation,
    ];
    let offsets = [
        allocations.cloud.byte_offset() as u64,
        allocations.surface.byte_offset() as u64,
        allocations.shape.byte_offset() as u64,
    ];
    let sizes = [
        allocations.cloud.byte_size() as u64,
        allocations.surface.byte_size() as u64,
        allocations.shape.byte_size() as u64,
    ];
    unsafe { allocator.flush_allocations(flush_allocations, Some(&offsets), Some(&sizes)) }?;

    Ok(())
}

fn upload(model: &mut Model, device: &Device, allocations: Allocations) -> Result<()> {
    // wait & reset `ready_to_add` (last "add" operation)
    let fences = [model.ready_to_add];
    unsafe { device.wait_for_fences(&fences, true, u64::MAX) }?;
    unsafe { device.reset_fences(&fences) }?;

    // reset transfer command pool
    unsafe {
        device.reset_command_pool(
            model.transfer_command_pool,
            vk::CommandPoolResetFlags::empty(),
        )
    }?;

    // record `add`
    record_add(model, device, allocations)?;

    // submit `add` : signal `ready_to_add` when done
    let command_buffer_info = vk::CommandBufferSubmitInfo::default().command_buffer(model.add);
    let command_buffer_infos = [command_buffer_info];
    let submit = vk::SubmitInfo2::default().command_buffer_infos(&command_buffer_infos);
    let submits = [submit];
    unsafe {
        device.queue_submit2(
            device.queue(QueueRoleFlags::TRANSFER).vk_queue,
            &submits,
            model.ready_to_add,
        )
    }?;

    Ok(())
}

fn record_add(model: &mut Model, device: &Device, allocations: Allocations) -> Result<()> {
    // begin
    let begin_info = vk::CommandBufferBeginInfo::default();
    unsafe { device.begin_command_buffer(model.add, &begin_info) }?;

    // copy visible cloud
    let region = vk::BufferCopy::default()
        .src_offset(allocations.cloud.byte_offset() as u64)
        .dst_offset(allocations.visible_cloud.byte_offset() as u64)
        .size(allocations.visible_cloud.byte_size() as u64);
    let regions = [region];
    unsafe {
        device.cmd_copy_buffer(
            model.add,
            model.clouds.full.buffer,
            model.clouds.visible.buffer,
            &regions,
        )
    };

    // copy visible surface
    let region = vk::BufferCopy::default()
        .src_offset(allocations.surface.byte_offset() as u64)
        .dst_offset(allocations.visible_surface.byte_offset() as u64)
        .size(allocations.visible_surface.byte_size() as u64);
    let regions = [region];
    unsafe {
        device.cmd_copy_buffer(
            model.add,
            model.surfaces.full.buffer,
            model.surfaces.visible.buffer,
            &regions,
        )
    };

    // copy shape info
    let region = vk::BufferCopy::default()
        .src_offset((allocations.shape.byte_offset() + offset_of!(Shape, info)) as u64)
        .dst_offset(allocations.shape_info.byte_offset() as u64)
        .size(allocations.shape_info.byte_size() as u64);
    let regions = [region];
    unsafe {
        device.cmd_copy_buffer(
            model.add,
            model.shapes.full.buffer,
            model.shapes.info.buffer,
            &regions,
        )
    };

    // copy shape diic
    let region = vk::BufferCopy::default()
        .src_offset((allocations.shape.byte_offset() + offset_of!(Shape, diic)) as u64)
        .dst_offset(allocations.shape_diic.byte_offset() as u64)
        .size(allocations.shape_diic.byte_size() as u64);
    let regions = [region];
    unsafe {
        device.cmd_copy_buffer(
            model.add,
            model.shapes.full.buffer,
            model.shapes.diic.buffer,
            &regions,
        )
    };

    // end
    unsafe { device.end_command_buffer(model.add) }?;

    Ok(())
}
