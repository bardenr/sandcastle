//! Native window lifecycle and input handling.

mod renderer;

use std::collections::HashSet;
use std::sync::Arc;

use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowAttributes, WindowId};

#[derive(Default)]
struct State {
    window: Option<Arc<Window>>,
    gpu: Option<renderer::State>,
    pressed_keys: HashSet<KeyCode>,
    log_input: bool,
}

pub(crate) fn run() {
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let mut state = State {
        log_input: std::env::var_os("SANDCASTLE_LOG_INPUT").as_deref() == Some("1".as_ref()),
        ..State::default()
    };
    event_loop.run_app(&mut state).expect("Event loop failed");
}

// winit requires this callback interface; state is kept in a plain struct.
impl ApplicationHandler for State {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let attrs = WindowAttributes::default()
                .with_title("Platform")
                .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));
            match event_loop.create_window(attrs) {
                Ok(window) => {
                    let window = Arc::new(window);
                    self.gpu = Some(renderer::init(Arc::clone(&window)));
                    window.request_redraw();
                    self.window = Some(window);
                }
                Err(e) => {
                    eprintln!("Failed to create window: {e}");
                    event_loop.exit();
                }
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                if self.log_input {
                    println!("Close requested");
                }
                event_loop.exit();
            }

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key_code),
                        state,
                        repeat,
                        ..
                    },
                ..
            } => {
                match state {
                    ElementState::Pressed => {
                        if !repeat {
                            if self.log_input {
                                println!("Key pressed: {key_code:?}");
                            }
                            self.pressed_keys.insert(key_code);
                        }
                    }
                    ElementState::Released => {
                        if self.log_input {
                            println!("Key released: {key_code:?}");
                        }
                        self.pressed_keys.remove(&key_code);
                    }
                }

                if key_code == KeyCode::Escape {
                    event_loop.exit();
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                if self.log_input {
                    println!("Mouse moved: ({:.1}, {:.1})", position.x, position.y);
                }
            }

            WindowEvent::MouseInput { state, button, .. } => {
                if self.log_input {
                    let action = match state {
                        ElementState::Pressed => "pressed",
                        ElementState::Released => "released",
                    };
                    println!("Mouse {action}: {button:?}");
                }
            }

            WindowEvent::Touch(touch) => {
                if self.log_input {
                    println!(
                        "Touch {:?} id={} at ({:.1}, {:.1})",
                        touch.phase, touch.id, touch.location.x, touch.location.y
                    );
                }
            }

            WindowEvent::Resized(size) => {
                if self.log_input {
                    println!("Resized: {}x{}", size.width, size.height);
                }
                if let Some(gpu) = &mut self.gpu
                    && renderer::resize(gpu, size.width, size.height)
                    && let Some(window) = &self.window
                {
                    window.request_redraw();
                }
            }

            WindowEvent::Occluded(false) => {
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }

            WindowEvent::RedrawRequested => {
                if let (Some(gpu), Some(window)) = (&self.gpu, &self.window) {
                    renderer::render(gpu, window);
                }
            }

            _ => {}
        }
    }
}
