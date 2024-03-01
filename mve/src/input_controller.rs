use std::collections::HashMap;

use cgmath::Vector2;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

pub struct InputMouse {
    mouse_buttons: u8,
    mouse_delta: Vector2<f32>,
}

pub struct InputKeyboard {
    pub key_map: HashMap<VirtualKeyCode, bool>,
}

pub struct InputController {
    pub(crate) mouse: InputMouse,
    pub(crate) keyboard: InputKeyboard,
}

impl InputController {
    pub fn new() -> Self {
        Self {
            mouse: InputMouse {
                mouse_delta: Vector2::new(0.0, 0.0),
                mouse_buttons: 0,
            },
            keyboard: InputKeyboard {
                key_map: HashMap::new(),
            },
        }
    }

    pub(crate) fn engine_mouse_move(&mut self, mouse_delta: Vector2<f32>) {
        self.mouse.mouse_delta = mouse_delta;
    }

    pub(crate) fn engine_device_button(&mut self, button: u32, state: ElementState) {
        // TODO: There's probably a more efficient way to do this
        if state == ElementState::Pressed {
            self.mouse.mouse_buttons |= 1 << button;
        } else {
            self.mouse.mouse_buttons &= !(1 << button);
        }
    }

    pub(crate) fn engine_key_input(&mut self, key_input: KeyboardInput) {
        if key_input.virtual_keycode.is_none() {
            return;
        }
        self.keyboard.key_map.insert(
            key_input.virtual_keycode.unwrap(),
            key_input.state == winit::event::ElementState::Pressed,
        );
    }

    //pub(crate) fn mouse_input(
    //    &mut self,
    //    _state: winit::event::ElementState,
    //    _button: winit::event::MouseButton,
    //) {
    //    let mut v: u8 = 0;
    //    if _state == winit::event::ElementState::Pressed {
    //        v |= 1 << _button as u16;
    //    } else {
    //        println!("Mouse button released");
    //    }
    //}
}
