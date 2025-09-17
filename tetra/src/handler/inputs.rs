use winit::{event::ElementState, keyboard::KeyCode};

const TRANSLATE_SENSITIVITY: f32 = 0.06;
const ROTATE_SENSITIVITY: f32 = 0.005;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct InputHandler {
    pub mouse_delta: (f64, f64),
    key_w: f32,
    key_a: f32,
    key_s: f32,
    key_d: f32,
    key_space: f32,
    key_shift_left: f32,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Update
impl InputHandler {
    pub fn key(&mut self, code: KeyCode, state: ElementState) {
        if state == ElementState::Pressed {
            match code {
                KeyCode::KeyW => self.key_w = 1.,
                KeyCode::KeyA => self.key_a = 1.,
                KeyCode::KeyS => self.key_s = 1.,
                KeyCode::KeyD => self.key_d = 1.,
                KeyCode::Space => self.key_space = 1.,
                KeyCode::ShiftLeft => self.key_shift_left = 1.,
                _ => (),
            }
        } else {
            // state == ElementState::Released
            match code {
                KeyCode::KeyW => self.key_w = 0.,
                KeyCode::KeyA => self.key_a = 0.,
                KeyCode::KeyS => self.key_s = 0.,
                KeyCode::KeyD => self.key_d = 0.,
                KeyCode::Space => self.key_space = 0.,
                KeyCode::ShiftLeft => self.key_shift_left = 0.,
                _ => (),
            }
        }
    }

    pub fn mouse_movement(&mut self, delta: (f64, f64)) {
        self.mouse_delta.0 += delta.0;
        self.mouse_delta.1 += delta.1;
    }
}

/// Getters
impl InputHandler {
    pub fn forward(&self) -> f32 {
        (self.key_w - self.key_s) * TRANSLATE_SENSITIVITY
    }

    pub fn right(&self) -> f32 {
        (self.key_d - self.key_a) * TRANSLATE_SENSITIVITY
    }

    pub fn above(&self) -> f32 {
        (self.key_space - self.key_shift_left) * TRANSLATE_SENSITIVITY
    }

    pub fn mouse_up(&self) -> f32 {
        -self.mouse_delta.1 as f32 * ROTATE_SENSITIVITY
    }

    pub fn mouse_right(&self) -> f32 {
        self.mouse_delta.0 as f32 * ROTATE_SENSITIVITY
    }
}
