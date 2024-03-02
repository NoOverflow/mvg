use crate::{
    camera_controller::CameraController,
    input_controller::InputController,
    rendering::{
        geometry::chunk::Chunk,
        traits::renderable::{RenderPassData, Renderable},
    },
};
use cgmath::{Point3, Vector3};

pub struct World {
    pub chunks: Vec<Chunk>,
    pub camera_controller: CameraController,
    pub input_controller: InputController,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: vec![Chunk::new(Vector3::<f32>::new(0.0, 0.0, 0.0))],
            camera_controller: CameraController::new(Point3::<f32>::new(5.0, 5.0, 5.0)),
            input_controller: InputController::new(),
        }
    }
}

impl Renderable for World {
    fn prepare(&mut self, device: &wgpu::Device) {
        for chunk in &mut self.chunks {
            chunk.prepare(device);
        }
    }

    fn render<'a>(
        &self,
        render_pass: &mut wgpu::RenderPass<'a>,
        render_pass_data: &'a RenderPassData,
    ) {
        self.camera_controller
            .camera
            .render(render_pass, render_pass_data);
        for chunk in &self.chunks {
            chunk.render(render_pass, render_pass_data);
        }
    }
}
