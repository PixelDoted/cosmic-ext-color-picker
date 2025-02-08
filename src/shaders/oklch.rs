use cosmic::{
    iced::{wgpu, Rectangle},
    iced_wgpu::graphics::Viewport,
    iced_widget::shader::{self, Storage},
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
        _cursor: cosmic::iced_core::mouse::Cursor,
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
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        storage: &mut Storage,
        _bounds: &Rectangle,
        _viewport: &Viewport,
    ) {
        if !storage.has::<ShaderPipeline<Uniforms, M>>() {
            storage.store(ShaderPipeline::<Uniforms, M>::new(
                device,
                format,
                include_str!("oklch.wgsl"),
            ));
        }

        let pipeline = storage.get_mut::<ShaderPipeline<Uniforms, M>>().unwrap();
        pipeline.write(queue, &self.uniforms);
    }

    fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        storage: &Storage,
        target: &wgpu::TextureView,
        clip_bounds: &Rectangle<u32>,
    ) {
        let pipeline = storage.get::<ShaderPipeline<Uniforms, M>>().unwrap();
        pipeline.render(target, encoder, clip_bounds);
    }
}

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct Uniforms {
    lightness: f32,
    chroma: f32,
    hue: f32,
    mode: u32,
}
