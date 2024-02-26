use cgmath::{Deg, Rad, Zero};
use wgpu::{util::DeviceExt, Device};

use super::{
    camera::Camera,
    geometry::voxel::{self, Voxel},
};

pub struct RenderPassData {
    pub cube_vertex_buffer: wgpu::Buffer,
    pub cube_index_buffer: wgpu::Buffer,
    pub projection_buffer: wgpu::Buffer,
}

impl RenderPassData {
    fn generate_matrix(aspect_ratio: f32) -> cgmath::Matrix4<f32> {
        let camera = Camera {
            eye: (2.5, 2.5, 2.5).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: 1024 as f32 / 768 as f32,
            fovy: 75.0,
            znear: 1.0,
            zfar: 10.0,
        };

        camera.build_view_projection_matrix()
    }

    pub fn new(device: &Device) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&voxel::VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&voxel::INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let mx_total = Self::generate_matrix(1024 as f32 / 768 as f32);
        let mx_ref: &[f32; 16] = mx_total.as_ref();
        let projection_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(mx_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            cube_vertex_buffer: vertex_buffer,
            cube_index_buffer: index_buffer,
            projection_buffer,
        }
    }
}

pub trait Renderable {
    fn prepare(&mut self, device: &wgpu::Device);
    fn render<'a>(
        &self,
        render_pass: &mut wgpu::RenderPass<'a>,
        render_pass_data: &'a RenderPassData,
    );
}
