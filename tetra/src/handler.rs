//! Handles events and execution.
//!
//! Contains a `Handler` structure that :
//! - Implements winit `ApplicationHandler` trait to receive events.
//! - Manage execution logic (application and context are mostly passive).

pub mod custom_events;
mod inputs;

use std::mem::MaybeUninit;

use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, ElementState, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::CursorGrabMode,
};

use super::{application::*, context::*};

use custom_events::UserEvent;
use inputs::InputHandler;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

/////////////////////////////////////////////////////////////////////////////
// Structure
/////////////////////////////////////////////////////////////////////////////

pub struct Handler {
    init: bool,
    context: MaybeUninit<Context>,
    application: MaybeUninit<Application>,
    input_handler: InputHandler,
    pause: bool,
}

/////////////////////////////////////////////////////////////////////////////
// Implementations
/////////////////////////////////////////////////////////////////////////////

/// Uninit & Init & Destroy
impl Handler {
    pub fn uninit() -> Handler {
        Handler {
            init: false,
            context: MaybeUninit::uninit(),
            application: MaybeUninit::uninit(),
            input_handler: InputHandler::default(),
            pause: true,
        }
    }

    fn init_if_needed(&mut self, event_loop: &ActiveEventLoop) -> Result<()> {
        if !self.init {
            // create
            let context = Context::new(event_loop)?;
            let mut application =
                Application::create(&context.instance, &context.device, &context.allocator)?;

            // init
            application.initialize(&context.device, &context.allocator)?;

            // write
            self.context.write(context);
            self.application.write(application);
            self.init = true;
        }
        Ok(())
    }

    fn destroy_if_needed(&mut self) {
        if self.init {
            unsafe {
                // destroy
                self.application.assume_init_mut().destroy_once_idle(
                    &self.context.assume_init_ref().device,
                    &self.context.assume_init_ref().allocator,
                );

                // drop
                self.application.assume_init_drop();
                self.context.assume_init_drop();
            }
            self.init = false;
        }
    }
}

/// ApplicationHandler
impl ApplicationHandler<UserEvent> for Handler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(init_error) = self.init_if_needed(event_loop) {
            println!("{init_error}");
            event_loop.exit();
        }
    }

    fn exiting(&mut self, _: &ActiveEventLoop) {
        self.destroy_if_needed();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: winit::window::WindowId,
        event: WindowEvent,
    ) {
        // extract
        let context = unsafe { self.context.assume_init_mut() };
        let application = unsafe { self.application.assume_init_mut() };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // update camera
                application.renderer.rendering_camera.camera.translate(
                    self.input_handler.forward(),
                    self.input_handler.right(),
                    self.input_handler.above(),
                );
                application.renderer.rendering_camera.camera.rotate(
                    self.input_handler.mouse_up(),
                    self.input_handler.mouse_right(),
                );
                self.input_handler.mouse_delta = (0., 0.);

                // draw
                let need_recreation = application
                    .renderer
                    .draw(&context.device, &context.allocator, &application.model)
                    .unwrap();

                // recreate eventually
                if need_recreation {
                    application
                        .renderer
                        .recreate(&context.instance, &context.device, &context.allocator)
                        .unwrap();
                }

                // loop by requesting another `WindowEvent::RedrawRequested`
                context.window.request_redraw();
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event: key_event,
                is_synthetic: _,
            } => {
                // If press `Escape` toggle pause
                if let PhysicalKey::Code(KeyCode::Escape) = key_event.physical_key {
                    if key_event.state == ElementState::Pressed {
                        if !self.pause {
                            context
                                .window
                                .set_cursor_grab(CursorGrabMode::None)
                                .unwrap();
                            context.window.set_cursor_visible(true);
                            self.pause = true;
                        } else {
                            context
                                .window
                                .set_cursor_grab(CursorGrabMode::Confined)
                                .or_else(|_e| {
                                    context.window.set_cursor_grab(CursorGrabMode::Locked)
                                })
                                .unwrap();
                            context.window.set_cursor_visible(false);
                            self.pause = false;
                        }
                    }
                }
            }
            _ => (),
        }
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: winit::event::DeviceId, event: DeviceEvent) {
        if self.pause {
            return;
        }
        match event {
            // Forward key inputs to `input_handler`
            DeviceEvent::Key(raw_key_event) => {
                if let PhysicalKey::Code(key_code) = raw_key_event.physical_key {
                    self.input_handler.key(key_code, raw_key_event.state);
                }
            }
            // Forward mouse motions to `input_handler`
            DeviceEvent::MouseMotion { delta } => self.input_handler.mouse_movement(delta),
            _ => (),
        }
    }

    fn user_event(&mut self, _: &ActiveEventLoop, _: UserEvent) {}
}
