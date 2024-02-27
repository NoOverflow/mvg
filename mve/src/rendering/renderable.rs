use cgmath::{Deg, Matrix4, Rad, SquareMatrix, Transform3, Zero};
use wgpu::{util::DeviceExt, Device, Queue};

use super::{
    camera::Camera,
    geometry::{
        transform::Transform,
        voxel::{self, Voxel},
    },
};

pub struct RenderPassData {
    pub cube_vertex_buffer: wgpu::Buffer,
    pub cube_index_buffer: wgpu::Buffer,
    pub projection_buffer: wgpu::Buffer,
    pub transform_buffer: wgpu::Buffer,
    pub queue: wgpu::Queue,
}

impl RenderPassData {
    fn camera_matrix(aspect_ratio: f32) -> cgmath::Matrix4<f32> {
        let camera = Camera {
            eye: (2.5, 2.5, 2.5).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: aspect_ratio,
            fovy: 75.0,
            znear: 1.0,
            zfar: 10.0,
        };

        camera.build_view_projection_matrix()
    }

    pub fn new(device: &Device, queue: Queue) -> Self {
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

        let mx_total = Self::camera_matrix(1024 as f32 / 768 as f32);
        let mx_ref: &[f32; 16] = mx_total.as_ref();
        let projection_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(mx_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let default_transform = Matrix4::<f32>::identity()
            * Matrix4::from_translation(cgmath::Vector3::new(-10.0, 0.0, 0.0));
        let default_transform_ref: &[f32; 16] = default_transform.as_ref();
        let transform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(default_transform_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            cube_vertex_buffer: vertex_buffer,
            cube_index_buffer: index_buffer,
            projection_buffer,
            transform_buffer,
            queue,
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
