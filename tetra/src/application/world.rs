use glam::{Quat, Vec3};
use vk_mem::Allocator;

use crate::context::Device;

use super::model::*;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct World {}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Generate
impl World {
    pub fn generate(
        &self,
        device: &Device,
        allocator: &Allocator,
        model: &mut Model,
    ) -> Result<()> {
        let cube_cloud = vec![
            Vertex::new(Vec3::new(0., 0., 0.)),
            Vertex::new(Vec3::new(0., 0., 1.)),
            Vertex::new(Vec3::new(0., 1., 0.)),
            Vertex::new(Vec3::new(0., 1., 1.)),
            Vertex::new(Vec3::new(1., 0., 0.)),
            Vertex::new(Vec3::new(1., 0., 1.)),
            Vertex::new(Vec3::new(1., 1., 0.)),
            Vertex::new(Vec3::new(1., 1., 1.)),
        ];

        // BC are always diagonals
        let cube_surface = vec![
            // top
            Triangle { a: 2, b: 6, c: 3 },
            Triangle { a: 7, b: 3, c: 6 },
            // bottom
            Triangle { a: 4, b: 0, c: 5 },
            Triangle { a: 1, b: 5, c: 0 },
            // left
            Triangle { a: 2, b: 3, c: 0 },
            Triangle { a: 1, b: 0, c: 3 },
            // right
            Triangle { a: 7, b: 6, c: 5 },
            Triangle { a: 4, b: 5, c: 6 },
            // front
            Triangle { a: 7, b: 5, c: 3 },
            Triangle { a: 1, b: 3, c: 5 },
            // back
            Triangle { a: 6, b: 2, c: 4 },
            Triangle { a: 0, b: 4, c: 2 },
        ];

        model.add(
            device,
            allocator,
            ShapeData {
                cloud: cube_cloud.clone(),
                visible_cloud_len: cube_cloud.len(),
                surface: cube_surface.clone(),
                visible_surface_len: cube_surface.len(),
                position: Vec3::new(0., 0., 0.),
                scale: 1.,
                orientation: Quat::IDENTITY,
            },
        )?;
        model.add(
            device,
            allocator,
            ShapeData {
                cloud: cube_cloud.clone(),
                visible_cloud_len: cube_cloud.len(),
                surface: cube_surface.clone(),
                visible_surface_len: cube_surface.len(),
                position: Vec3::new(5., 0., 0.),
                scale: 2.,
                orientation: Quat::from_rotation_x(1.),
            },
        )?;
        model.add(
            device,
            allocator,
            ShapeData {
                cloud: cube_cloud.clone(),
                visible_cloud_len: cube_cloud.len(),
                surface: cube_surface.clone(),
                visible_surface_len: cube_surface.len(),
                position: Vec3::new(10., 0., 0.),
                scale: 0.5,
                orientation: Quat::from_rotation_y(2.),
            },
        )?;

        Ok(())
    }
}
