use glam::{Mat4, Quat, Vec3};

const FORWARD: Vec3 = Vec3::NEG_Z;
const RIGHT: Vec3 = Vec3::X;
const ABOVE: Vec3 = Vec3::Y;

/////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////

pub struct Camera {
    pub position: Vec3,
    pub orientation: Quat,
    pub projection: Mat4,
}

/////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////

/// - `fov_y_radians` = angle between top and bottom plane (x fov is derived using `aspect_ratio`).
/// - `aspect_ratio` = width / height.
/// - `z_near` & `z_far` = distances to z planes.
pub struct Frustrum {
    pub fov_y_radians: f32,
    pub aspect_ratio: f32,
    pub z_near: f32,
    pub z_far: f32,
}

/// New
impl Camera {
    pub fn new(position: Vec3, orientation: Quat, frustrum: Frustrum) -> Self {
        Self {
            position,
            orientation,
            projection: Mat4::perspective_rh(
                frustrum.fov_y_radians,
                frustrum.aspect_ratio,
                frustrum.z_near,
                frustrum.z_far,
            ),
        }
    }
}

/// Update
impl Camera {
    pub fn translate(&mut self, forward: f32, right: f32, above: f32) {
        let relative_step = forward * FORWARD + right * RIGHT + above * ABOVE;
        let absolute_step = self.orientation * relative_step;
        self.position += absolute_step;
    }

    pub fn rotate(&mut self, up_angle_radians: f32, right_angle_radians: f32) {
        let pitch = Quat::from_axis_angle(RIGHT, up_angle_radians); // pitch = up-down
        let yaw = Quat::from_axis_angle(-ABOVE, right_angle_radians); // yaw = left-right
        self.orientation = ((yaw * self.orientation) * pitch).normalize();
    }

    pub fn redo_projection(&mut self, frustrum: Frustrum) {
        self.projection = Mat4::perspective_rh(
            frustrum.fov_y_radians,
            frustrum.aspect_ratio,
            frustrum.z_near,
            frustrum.z_far,
        );
    }
}

/// View
impl Camera {
    pub fn view(&self) -> Mat4 {
        let eye = self.position;
        let center = eye + (self.orientation * FORWARD);
        let up = self.orientation * ABOVE;
        Mat4::look_at_rh(eye, center, up)
    }
}
