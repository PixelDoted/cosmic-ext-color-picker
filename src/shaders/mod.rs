pub mod hsv;
pub mod oklab;
pub mod oklch;

use std::marker::PhantomData;

use cosmic::iced::{wgpu, widget, Rectangle};

pub struct ShaderPipeline<T, const ID: u32> {
    pipeline: Option<wgpu::RenderPipeline>,
    pipeline_layout: wgpu::PipelineLayout,
    bind_group: wgpu::BindGroup,
    data: wgpu::Buffer,
    texture_format: wgpu::TextureFormat,
    phantom: PhantomData<T>,
}

impl<T: bytemuck::Pod + Send + Sync, const ID: u32> widget::shader::Pipeline
    for ShaderPipeline<T, ID>
{
    fn new(device: &wgpu::Device, _queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let data_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("shader data buffer"),
            size: std::mem::size_of::<T>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("shader uniform bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shader uniform bind group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: data_buffer.as_entire_binding(),
            }],
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("data pipeline layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            immediate_size: 0,
        });

        Self {
            pipeline: None,
            pipeline_layout: layout,
            bind_group: uniform_bind_group,
            data: data_buffer,
            texture_format: format,
            phantom: PhantomData,
        }
    }
}

impl<T: bytemuck::Pod + Send + Sync, const ID: u32> ShaderPipeline<T, ID> {
    pub fn initialize(&mut self, device: &wgpu::Device, _queue: &wgpu::Queue, shader: &str) {
        let vertex_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("graph vertex shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                "vertex.wgsl"
            ))),
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("graph shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(shader)),
        });

        self.pipeline = Some(
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("graph pipeline"),
                layout: Some(&self.pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vertex_shader,
                    entry_point: Some("vs_main"),
                    buffers: &[],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: self.texture_format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                cache: None,
                multiview_mask: None,
            }),
        );
    }

    pub fn write(&self, queue: &wgpu::Queue, data: &T) {
        queue.write_buffer(&self.data, 0, bytemuck::bytes_of(data));
    }

    pub fn render(
        &self,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        viewport: &Rectangle<u32>,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("shader.pipeline.pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        pass.set_pipeline(self.pipeline.as_ref().unwrap());
        #[allow(clippy::cast_precision_loss)]
        pass.set_viewport(
            viewport.x as f32,
            viewport.y as f32,
            viewport.width as f32,
            viewport.height as f32,
            0.0,
            1.0,
        );
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.draw(0..3, 0..1);
    }
}
