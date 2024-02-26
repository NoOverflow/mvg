use std::borrow::Cow;

use wgpu::{PipelineLayout, TextureFormat};

pub struct Pipeline {
    layout: PipelineLayout,
    shader_module: wgpu::ShaderModule,
    pub render_pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    // TODO: This is a naive implementation, add support for more pipeline render parameters (ex: depth_stencil) (Use a builder pattern)
    pub fn new(
        device: &wgpu::Device,
        swpchn_format: TextureFormat,
        shader_data: &'static str,
    ) -> Self {
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader_data)),
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
                targets: &[Some(swpchn_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self {
            layout,
            shader_module,
            render_pipeline,
        }
    }
}
