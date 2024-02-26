use crate::rendering::{
    geometry::chunk::Chunk,
    renderable::{RenderPassData, Renderable},
};
use cgmath::Vector3;

pub struct World {
    pub chunks: Vec<Chunk>,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: vec![Chunk::new(Vector3::<f32>::new(0.0, 0.0, 0.0))],
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
        for chunk in &self.chunks {
            chunk.render(render_pass, render_pass_data);
        }
    }
}
