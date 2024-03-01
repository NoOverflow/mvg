use std::time::Instant;

use cgmath::{Vector2, Vector3, Zero};
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod camera_controller;
mod input_controller;
mod rendering;
mod world;

use rendering::renderer::State;
use world::World;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let mut last_frame: Instant = Instant::now();
    let mut engine_state = State::new(window, World::new()).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == engine_state.window().id() => {
            if !engine_state.input(event) {
                match event {
                    WindowEvent::MouseInput {
                        device_id,
                        state,
                        button,
                        ..
                    } => {}
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        engine_state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        engine_state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        Event::DeviceEvent { device_id, event } => match event {
            winit::event::DeviceEvent::MouseMotion { delta } => {
                engine_state
                    .world
                    .input_controller
                    .engine_mouse_move(Vector2 {
                        x: delta.0 as f32,
                        y: delta.1 as f32,
                    });
            }
            winit::event::DeviceEvent::Button { button, state } => {
                engine_state
                    .world
                    .input_controller
                    .engine_device_button(button, state);
            }
            winit::event::DeviceEvent::Key(input) => {
                engine_state.world.input_controller.engine_key_input(input);
            }
            _ => {}
        },
        Event::RedrawRequested(window_id) if window_id == engine_state.window().id() => {
            let delta = last_frame.elapsed();

            last_frame = Instant::now();
            engine_state.update();
            match engine_state.render(delta) {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                    engine_state.resize(engine_state.size)
                }
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
            }
        }
        Event::RedrawEventsCleared => {
            engine_state.window().request_redraw();
        }
        _ => {}
    });
}
