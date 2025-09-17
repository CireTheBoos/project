use glam::Vec3;

/////////////////////////////////////////////////////////////////////////////

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub position_in_shape: Vec3,
    pub padding: f32,
}

/////////////////////////////////////////////////////////////////////////////

impl Vertex {
    pub fn new(position_in_shape: Vec3) -> Self {
        Self {
            position_in_shape,
            padding: 0.,
        }
    }
}
