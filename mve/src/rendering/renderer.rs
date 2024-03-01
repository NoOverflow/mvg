use std::{borrow::Cow, iter, time::Duration};
use wgpu::{Adapter, Device, Surface, TextureFormat};
use winit::{event::*, window::Window};

use crate::world::World;

use super::{primitives::vertex::Vertex, renderable::Renderable};

pub struct Pipeline {
    render_pipeline: wgpu::RenderPipeline,
}

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    render_pass_data: crate::rendering::renderable::RenderPassData,

    window: Window,
    pub world: crate::World,
    pipeline: Pipeline,

    // Bind groups
    projection_bind_group: wgpu::BindGroup,
    transform_bind_group: wgpu::BindGroup,
}

impl State {
    fn create_pipeline(
        device: &wgpu::Device,
        swpchn_format: TextureFormat,
        shader_data: &'static str,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
    ) -> Pipeline {
        let shader_module: wgpu::ShaderModule =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(shader_data)),
            });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: bind_group_layouts,
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
                targets: &[Some(swpchn_format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Pipeline { render_pipeline }
    }

    fn configure_surface(
        size: winit::dpi::PhysicalSize<u32>,
        device: &Device,
        surface: &Surface,
        adapter: &Adapter,
    ) -> wgpu::SurfaceConfiguration {
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);
        config
    }

    pub async fn new(window: Window, world: World) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();
        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_capabilities.formats[0];
        let render_pass_data = crate::rendering::renderable::RenderPassData::new(&device, queue);
        let config = Self::configure_surface(size, &device, &surface, &adapter);

        let projection_buffer_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                    count: None,
                }],
            });

        let projection_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &projection_buffer_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: render_pass_data.projection_buffer.as_entire_binding(),
            }],
            label: None,
        });

        let transform_buffer_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                    count: None,
                }],
            });

        let transform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &transform_buffer_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: render_pass_data.transform_buffer.as_entire_binding(),
            }],
            label: None,
        });

        let pipeline = Self::create_pipeline(
            &device,
            swapchain_format,
            include_str!("shaders/default.wgsl"),
            &[
                &projection_buffer_bind_group_layout,
                &transform_buffer_bind_group_layout,
            ],
        );

        window.set_cursor_visible(false);
        Self {
            surface,
            device,
            config,
            size,
            window,
            world,
            projection_bind_group,
            render_pass_data,
            pipeline,
            transform_bind_group,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    #[allow(unused_variables)]
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self, delta: Duration) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            render_pass.set_pipeline(&self.pipeline.render_pipeline);
            render_pass.set_bind_group(0, &self.projection_bind_group, &[]);
            render_pass.set_bind_group(1, &self.transform_bind_group, &[]);

            self.world
                .render(&mut render_pass, &mut self.render_pass_data);
        }

        self.render_pass_data
            .queue
            .submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}
