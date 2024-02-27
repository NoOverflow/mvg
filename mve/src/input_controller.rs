use cgmath::Vector2;

pub struct InputController {
    pub mouse_delta: Vector2<f32>,
    pub mouse_position: Vector2<f32>,
}

impl InputController {
    pub fn new() -> Self {
        Self {
            mouse_delta: Vector2::new(0.0, 0.0),
            mouse_position: Vector2::new(0.0, 0.0),
        }
    }

    pub(crate) fn mouse_move(&mut self, mouse_delta: Vector2<f32>) {
        self.mouse_delta = mouse_delta;
        // self.mouse_position = mouse_position;
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
