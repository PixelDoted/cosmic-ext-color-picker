use cosmic::iced::{
    wgpu,
    widget::shader::{self, Storage, Viewport},
    Rectangle,
};

use crate::shaders::ShaderPipeline;

// ---- Shader ----
pub struct ColorGraph<const MODE: u32> {
    pub lightness: f32,
    pub chroma: f32,
    pub hue: f32,
}

impl<const M: u32, Message> shader::Program<Message> for ColorGraph<M> {
    type State = ();
    type Primitive = Primitive<M>;

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: cosmic::iced::mouse::Cursor,
        _bounds: cosmic::iced::Rectangle,
    ) -> Self::Primitive {
        Primitive::<M>::new(self.lightness, self.chroma, self.hue)
    }
}

#[derive(Debug)]
pub struct Primitive<const M: u32> {
    uniforms: Uniforms,
}

impl<const M: u32> Primitive<M> {
    pub fn new(lightness: f32, chroma: f32, hue: f32) -> Self {
        Self {
            uniforms: Uniforms {
                lightness,
                chroma,
                hue,
                mode: M,
            },
        }
    }
}

impl<const M: u32> shader::Primitive for Primitive<M> {
    type Pipeline = ShaderPipeline<Uniforms, M>;

    fn prepare(
        &self,
        pipeline: &mut Self::Pipeline,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _bounds: &Rectangle,
        _viewport: &Viewport,
    ) {
        pipeline.initialize(device, queue, include_str!("oklch.wgsl"));
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
    lightness: f32,
    chroma: f32,
    hue: f32,
    mode: u32,
}
