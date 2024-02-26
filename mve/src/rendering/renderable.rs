use cgmath::{Deg, Rad, Zero};
use wgpu::{util::DeviceExt, Device};

use super::geometry::voxel::{self, Voxel};

pub struct RenderPassData {
    pub cube_vertex_buffer: wgpu::Buffer,
    pub cube_index_buffer: wgpu::Buffer,
    pub transform_buffer: wgpu::Buffer,
}

impl RenderPassData {
    fn generate_matrix(aspect_ratio: f32) -> cgmath::Matrix4<f32> {
        let projection = cgmath::perspective(cgmath::Deg(90.0), aspect_ratio, 1.0, 10.0);
        let view = cgmath::Matrix4::look_at_rh(
            cgmath::Point3::new(1.5f32, -5.0, 3.0),
            cgmath::Point3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::unit_z(),
        );
        projection * view
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
        let uniform_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(mx_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            cube_vertex_buffer: vertex_buffer,
            cube_index_buffer: index_buffer,
            transform_buffer: uniform_buf,
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
