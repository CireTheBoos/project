//! Game logic.
//!
//! Contains `Application` structure with :
//! - Rendering logic.
//! - Model logic.

pub mod model;
pub mod renderer;
pub mod world;

use vk_mem::Allocator;

use crate::context::{Device, Instance};

use model::Model;
use renderer::Renderer;
use world::World;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct Application {
    pub model: Model,
    pub renderer: Renderer,
    pub world: World,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Create & RecreateSwapchain & Destroy
impl Application {
    pub fn create(
        instance: &Instance,
        device: &Device,
        allocator: &Allocator,
    ) -> Result<Application> {
        Ok(Application {
            model: Model::create(device, allocator)?,
            renderer: Renderer::create(instance, device, allocator)?,
            world: World {},
        })
    }

    pub fn initialize(&mut self, device: &Device, allocator: &Allocator) -> Result<()> {
        self.world.generate(device, allocator, &mut self.model)
    }

    pub fn destroy_once_idle(&mut self, device: &Device, allocator: &Allocator) {
        unsafe { device.device_wait_idle().unwrap() };
        self.renderer.destroy(device, allocator);
        self.model.destroy(device, allocator);
    }
}
