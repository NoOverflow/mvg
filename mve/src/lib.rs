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
    let mut state = State::new(window, World::new()).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window().id() => {
            if !state.input(event) {
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
                        state.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        Event::DeviceEvent { device_id, event } => match event {
            winit::event::DeviceEvent::MouseMotion { delta } => {
                state.world.input_controller.mouse_move(Vector2 {
                    x: delta.0 as f32,
                    y: delta.1 as f32,
                });

                // Note: temporary
                state.world.camera_controller.rotate_camera(Vector2 {
                    x: delta.0 as f32,
                    y: delta.1 as f32,
                })
            }
            winit::event::DeviceEvent::Button { button, state } => {
                if button == 1 {
                    match state {
                        ElementState::Pressed => {}
                        ElementState::Released => {}
                    }
                }
            }
            winit::event::DeviceEvent::Key(input) => {
                let mut move_vector = Vector3::<f32>::zero();

                match input.virtual_keycode {
                    Some(VirtualKeyCode::Z) => {
                        move_vector.z = -1.0;
                    }
                    Some(VirtualKeyCode::S) => {
                        move_vector.z = 1.0;
                    }
                    Some(VirtualKeyCode::Q) => {
                        move_vector.x = -1.0;
                    }
                    Some(VirtualKeyCode::D) => {
                        move_vector.x = 1.0;
                    }
                    Some(VirtualKeyCode::LShift) => {
                        move_vector.y = 1.0;
                    }
                    Some(VirtualKeyCode::LControl) => {
                        move_vector.y = -1.0;
                    }
                    _ => {}
                }
                state.world.camera_controller.translate_camera(move_vector);
            }
            _ => {}
        },
        Event::RedrawRequested(window_id) if window_id == state.window().id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                    state.resize(state.size)
                }
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
            }
        }
        Event::RedrawEventsCleared => {
            state.window().request_redraw();
        }
        _ => {}
    });
}
