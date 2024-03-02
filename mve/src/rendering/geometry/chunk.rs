use super::voxel::Voxel;
use crate::rendering::traits::renderable::RenderPassData;
use crate::rendering::traits::renderable::Renderable;
use cgmath::{Quaternion, Vector3, Zero};
use wgpu::util::DeviceExt;

const CHUNK_SIZE: usize = 1;

pub struct Chunk {
    pub voxels: Vec<Voxel>,
    pub position: Vector3<f32>,
}

impl Chunk {
    pub fn new(position: Vector3<f32>) -> Self {
        let mut voxels = Vec::with_capacity(CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE);

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    voxels.push(Voxel::new(
                        Vector3::<f32>::new(
                            position.x + x as f32,
                            position.y + y as f32,
                            position.z + z as f32,
                        ),
                        Quaternion::<f32>::zero(),
                    ));
                }
            }
        }
        Self { voxels, position }
    }
}

impl Renderable for Chunk {
    fn prepare(&mut self, device: &wgpu::Device) {
        for voxel in &mut self.voxels {
            voxel.prepare(device);
        }
    }

    fn render<'a>(
        &self,
        render_pass: &mut wgpu::RenderPass<'a>,
        render_pass_data: &'a RenderPassData,
    ) {
        // TODO: Optimise, this is just a PoC
        for voxel in &self.voxels {
            voxel.render(render_pass, render_pass_data);
        }
    }
}
