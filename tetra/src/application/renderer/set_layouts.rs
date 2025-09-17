mod scene;

use crate::context::Device;

pub use scene::Scene;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct SetLayouts {
    pub scene: Scene,
}

impl SetLayouts {
    pub fn create(device: &Device) -> Result<Self> {
        Ok(Self {
            scene: Scene::create(device)?,
        })
    }

    pub fn destroy(&mut self, device: &Device) {
        self.scene.destroy(device);
    }
}
