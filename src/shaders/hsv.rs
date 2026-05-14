use cosmic::iced::{
    wgpu,
    widget::shader::{self, Storage, Viewport},
    Rectangle,
};

use crate::shaders::ShaderPipeline;

// ---- Shader ----
pub struct ColorGraph {
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
}

impl<Message> shader::Program<Message> for ColorGraph {
    type State = ();
    type Primitive = Primitive;

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: cosmic::iced::mouse::Cursor,
        _bounds: cosmic::iced::Rectangle,
    ) -> Self::Primitive {
        Primitive::new(self.hue, self.saturation, self.value)
    }
}

#[derive(Debug)]
pub struct Primitive {
    uniforms: Uniforms,
}

impl Primitive {
    pub fn new(hue: f32, saturation: f32, value: f32) -> Self {
        Self {
            uniforms: Uniforms {
                hue,
                saturation,
                value,
            },
        }
    }
}

impl shader::Primitive for Primitive {
    type Pipeline = ShaderPipeline<Uniforms, 0>;

    fn prepare(
        &self,
        pipeline: &mut Self::Pipeline,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _bounds: &Rectangle,
        _viewport: &Viewport,
    ) {
        pipeline.initialize(device, queue, include_str!("hsv.wgsl"));
        pipeline.write(queue, &self.uniforms);
    }

    fn render(
        &self,
        pipeline: &Self::Pipeline,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        clip_bounds: &Rectangle<u32>,
    ) {
        pipeline.render(target, encoder, clip_bounds);
    }
}

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Uniforms {
    hue: f32,
    saturation: f32,
    value: f32,
}
