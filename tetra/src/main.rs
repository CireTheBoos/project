//! Setup (prior to context).

mod application;
mod context;
mod handler;

use std::{thread, time::Duration};

use winit::event_loop::{ControlFlow, EventLoop};

use handler::{
    Handler,
    custom_events::{TICK_INTERVAL_MILLISECONDS, UserEvent},
};

/////////////////////////////////////////////////////////////////////////////

/// Setup :
/// 1. Create uninit handler.
/// 2. Create winit event loop.
/// 3. Spawn thread to send custom events via event loop proxy (like ticks).
/// 4. Run handler on event loop :
///     - => Event loop will call handler's `resumed(..)` method.
///     - => In `resumed(..)` : Init handler.
///
/// Execution :
/// 1. Event loop forwards events to handler's `ApplicationHandler` methods (see winit doc).
/// 2. Handler process events (using application & context).
///
/// Winit memo : "memos/winit.md" (help understand architecture choices).
fn main() {
    // uninit handler
    let mut handler = Handler::uninit();

    // event loop
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let event_loop_proxy = event_loop.create_proxy();

    // tick thread
    let tick_thread = thread::spawn(move || {
        loop {
            // send tick and test if event loop has closed
            if event_loop_proxy.send_event(UserEvent::Tick).is_ok() {
                thread::sleep(Duration::from_millis(TICK_INTERVAL_MILLISECONDS));
            } else {
                break;
            }
        }
    });

    // run
    event_loop.run_app(&mut handler).unwrap();

    // wait tick thread
    tick_thread.join().unwrap();
}
