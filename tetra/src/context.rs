//! Tools to execute complex work.
//!
//! Contains a `Context` structure with :
//! - Window.
//! - vulkan's `Instance` and `Device`.
//! - VMA allocator.

pub mod device;
mod entry;
mod instance;

use ash::vk;
use vk_mem::Allocator;
use winit::{
    event_loop::ActiveEventLoop,
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};

pub use device::Device;
use entry::Entry;
pub use instance::Instance;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

const WINDOW_TITLE: &str = "Tetra";

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

/// - Wraps vulkan context + window :
///     - `Instance` : Query hardware or external infos (about physical device or surface).
///     - `Device` : Execute vulkan functions.
///     - `Allocator` : Manage memory (buffer, image, flush/invalidate).
pub struct Context {
    pub window: Window,
    _entry: Entry,
    pub instance: Instance,
    pub surface: vk::SurfaceKHR,
    pub device: Device,
    pub allocator: Allocator,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// New
impl Context {
    pub fn new(event_loop: &ActiveEventLoop) -> Result<Self> {
        // window
        let window =
            event_loop.create_window(Window::default_attributes().with_title(WINDOW_TITLE))?;

        // entry
        let entry = Entry::new()?;

        // instance
        let instance = Instance::create(&entry, window.display_handle()?.as_raw())?;

        // surface
        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                window.display_handle()?.as_raw(),
                window.window_handle()?.as_raw(),
                None,
            )?
        };

        // device
        let device = Device::create(&instance, surface)?;

        // allocator
        let allocator = unsafe {
            vk_mem::Allocator::new(vk_mem::AllocatorCreateInfo::new(
                &instance,
                &device,
                device.physical_device,
            ))
        }?;

        Ok(Self {
            window,
            _entry: entry,
            instance,
            surface,
            device,
            allocator,
        })
    }
}

/// Drop
impl Drop for Context {
    fn drop(&mut self) {
        // allocator
        unsafe { std::ptr::drop_in_place(&mut self.allocator) };

        // device
        self.device.destroy();

        // surface
        unsafe {
            self.instance
                .surface_instance
                .destroy_surface(self.surface, None)
        };

        // instance
        self.instance.destroy();
    }
}
