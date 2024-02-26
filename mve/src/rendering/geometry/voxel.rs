use std::rc::Rc;

use wgpu::util::DeviceExt;

use crate::rendering::{
    primitives::vertex::{self, vertex, Vertex},
    renderable::{RenderPassData, Renderable},
};

#[derive(Clone)]
pub struct Voxel {
    pub position: cgmath::Vector3<f32>,
}

pub const VERTICES: &[Vertex] = &[
    // Top
    Vertex {
        pos: [-1.0, -1.0, 1.0, 1.0],
        tex_coord: [0.0, 0.0],
    },
    Vertex {
        pos: [1.0, -1.0, 1.0, 1.0],
        tex_coord: [1.0, 0.0],
    },
    Vertex {
        pos: [1.0, 1.0, 1.0, 1.0],
        tex_coord: [1.0, 1.0],
    },
    Vertex {
        pos: [-1.0, 1.0, 1.0, 1.0],
        tex_coord: [0.0, 1.0],
    },
    // Bottom
    Vertex {
        pos: [-1.0, 1.0, -1.0, 1.0],
        tex_coord: [1.0, 0.0],
    },
    Vertex {
        pos: [1.0, 1.0, -1.0, 1.0],
        tex_coord: [0.0, 0.0],
    },
    Vertex {
        pos: [1.0, -1.0, -1.0, 1.0],
        tex_coord: [0.0, 1.0],
    },
    Vertex {
        pos: [-1.0, -1.0, -1.0, 1.0],
        tex_coord: [1.0, 1.0],
    },
    // Right
    Vertex {
        pos: [1.0, -1.0, -1.0, 1.0],
        tex_coord: [0.0, 0.0],
    },
    Vertex {
        pos: [1.0, 1.0, -1.0, 1.0],
        tex_coord: [1.0, 0.0],
    },
    Vertex {
        pos: [1.0, 1.0, 1.0, 1.0],
        tex_coord: [1.0, 1.0],
    },
    Vertex {
        pos: [1.0, -1.0, 1.0, 1.0],
        tex_coord: [0.0, 1.0],
    },
    // Left
    Vertex {
        pos: [-1.0, -1.0, 1.0, 1.0],
        tex_coord: [1.0, 0.0],
    },
    Vertex {
        pos: [-1.0, 1.0, 1.0, 1.0],
        tex_coord: [0.0, 0.0],
    },
    Vertex {
        pos: [-1.0, 1.0, -1.0, 1.0],
        tex_coord: [0.0, 1.0],
    },
    Vertex {
        pos: [-1.0, -1.0, -1.0, 1.0],
        tex_coord: [1.0, 1.0],
    },
    // Front
    Vertex {
        pos: [1.0, 1.0, -1.0, 1.0],
        tex_coord: [1.0, 0.0],
    },
    Vertex {
        pos: [-1.0, 1.0, -1.0, 1.0],
        tex_coord: [0.0, 0.0],
    },
    Vertex {
        pos: [-1.0, 1.0, 1.0, 1.0],
        tex_coord: [0.0, 1.0],
    },
    Vertex {
        pos: [1.0, 1.0, 1.0, 1.0],
        tex_coord: [1.0, 1.0],
    },
    Vertex {
        pos: [1.0, -1.0, 1.0, 1.0],
        tex_coord: [0.0, 0.0],
    },
    Vertex {
        pos: [-1.0, -1.0, 1.0, 1.0],
        tex_coord: [1.0, 0.0],
    },
    Vertex {
        pos: [-1.0, -1.0, -1.0, 1.0],
        tex_coord: [1.0, 1.0],
    },
    Vertex {
        pos: [1.0, -1.0, -1.0, 1.0],
        tex_coord: [0.0, 1.0],
    },
];

pub const INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0, // top
    4, 5, 6, 6, 7, 4, // bottom
    8, 9, 10, 10, 11, 8, // right
    12, 13, 14, 14, 15, 12, // left
    16, 17, 18, 18, 19, 16, // front
    20, 21, 22, 22, 23, 20, // back
];

impl Voxel {
    pub fn new(position: cgmath::Vector3<f32>) -> Self {
        Self { position }
    }
}

impl Renderable for Voxel {
    fn prepare(&mut self, device: &wgpu::Device) {}

    fn render<'a>(
        &self,
        render_pass: &mut wgpu::RenderPass<'a>,
        render_pass_data: &'a RenderPassData,
    ) {
        render_pass.set_index_buffer(
            render_pass_data.cube_index_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.set_vertex_buffer(0, render_pass_data.cube_vertex_buffer.slice(..));
        render_pass.insert_debug_marker("Draw!");
        render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
    }
}
