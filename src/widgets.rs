use std::{f32::consts::FRAC_PI_2, ops::RangeInclusive};

use cosmic::{
    iced::{
        border,
        gradient::{ColorStop, Linear},
        mouse, touch, Background, Border, Color, Element, Event, Gradient, Length, Padding, Point,
        Rectangle, Shadow, Size,
    },
    iced_core::{layout, renderer, widget::tree},
    theme,
    widget::{self, Widget},
};

pub struct ColorBlock {
    color: Color,
    border: [bool; 4],
    width: Length,
    height: Length,
}

impl ColorBlock {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            border: [true; 4],
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    pub fn border(mut self, border: [bool; 4]) -> Self {
        self.border = border;
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

impl<'a, Message: 'a> From<ColorBlock> for cosmic::Element<'a, Message> {
    fn from(value: ColorBlock) -> Self {
        let ColorBlock {
            color,
            border,
            width,
            height,
        } = value;

        widget::container(widget::Space::with_height(height))
            .class(theme::Container::custom(move |theme| {
                let cosmic = theme.cosmic();
                let radius = cosmic.corner_radii.radius_xs;

                #[allow(clippy::cast_lossless)]
                cosmic::widget::container::Style {
                    background: Some(value.color.into()),
                    border: Border {
                        radius: [
                            radius[0] * (border[0] as i8) as f32,
                            radius[1] * (border[1] as i8) as f32,
                            radius[2] * (border[2] as i8) as f32,
                            radius[3] * (border[3] as i8) as f32,
                        ]
                        .into(),
                        color,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            }))
            .width(width)
            .into()
    }
}

pub fn color_block(color: Color) -> ColorBlock {
    ColorBlock::new(color)
}

pub fn color_slider<'a, Message>(
    range: RangeInclusive<f32>,
    value: f32,
    on_change: impl Fn(f32) -> Message + 'a,
    color_stops: &'static [ColorStop],
) -> cosmic::Element<'a, Message>
where
    Message: Clone + 'a,
{
    ColorSlider {
        value,
        range,
        background: Gradient::Linear(Linear::new(FRAC_PI_2).add_stops(color_stops.iter().copied())),
        scroll_steps: 0.01,
        on_change: Box::new(on_change),
    }
    .into()
}

pub struct ColorSlider<'a, Message> {
    value: f32,
    range: RangeInclusive<f32>,
    background: Gradient,
    scroll_steps: f32,
    on_change: Box<dyn Fn(f32) -> Message + 'a>,
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for ColorSlider<'_, Message>
where
    Renderer: renderer::Renderer,
{
    fn state(&self) -> cosmic::iced_core::widget::tree::State {
        tree::State::new(ColorSliderState { is_dragging: false })
    }

    fn size(&self) -> cosmic::iced::Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fixed(15.0),
        }
    }

    fn layout(
        &self,
        _tree: &mut cosmic::iced_core::widget::Tree,
        _renderer: &Renderer,
        limits: &cosmic::iced_core::layout::Limits,
    ) -> cosmic::iced_core::layout::Node {
        layout::Node::new(Size::new(limits.max().width, 15.0))
    }

    fn draw(
        &self,
        _tree: &cosmic::iced_core::widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: cosmic::iced_core::Layout<'_>,
        _cursor: cosmic::iced_core::mouse::Cursor,
        _viewport: &cosmic::iced::Rectangle,
    ) {
        let bounds = layout.bounds();
        let rail_bounds = bounds.shrink(Padding::new(0.0).top(3.0).bottom(3.0));

        let percent = (self.value - self.range.start()) / (self.range.end() - self.range.start());
        let handle_bounds = Rectangle {
            x: bounds.x + (bounds.width - bounds.height) * percent,
            y: bounds.y,
            width: bounds.height,
            height: bounds.height,
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: rail_bounds,
                border: border::rounded(10),
                shadow: Shadow {
                    ..Default::default()
                },
            },
            self.background,
        );

        renderer.fill_quad(
            renderer::Quad {
                bounds: handle_bounds,
                border: Border {
                    color: Color::WHITE,
                    width: 3.0,
                    radius: 10f32.into(),
                },
                shadow: Shadow::default(),
            },
            Background::Color(Color::TRANSPARENT),
        );
    }

    fn on_event(
        &mut self,
        tree: &mut cosmic::iced_core::widget::Tree,
        event: cosmic::iced::Event,
        layout: layout::Layout<'_>,
        cursor: cosmic::iced_core::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn cosmic::iced_core::Clipboard,
        shell: &mut cosmic::iced_core::Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> cosmic::iced_core::event::Status {
        let state = tree.state.downcast_mut::<ColorSliderState>();

        let mut change = |position: &Point| {
            let bounds = layout.bounds();
            let relative_x = (position.x - bounds.x) / (bounds.width - bounds.height);
            let percent = relative_x.clamp(0.0, 1.0);

            self.value = self.range.start() * (1.0 - percent) + self.range.end() * percent;
            shell.publish((self.on_change)(self.value));
        };

        match &event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if let Some(position) = cursor.position_over(layout.bounds()) {
                    state.is_dragging = true;
                    change(&position);
                    return cosmic::iced_core::event::Status::Captured;
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. }) => {
                if state.is_dragging {
                    state.is_dragging = false;
                    return cosmic::iced_core::event::Status::Captured;
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                if state.is_dragging {
                    if let Some(position) = cursor.position().as_ref() {
                        change(position);
                    }

                    return cosmic::iced_core::event::Status::Captured;
                }
            }

            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if !state.is_dragging && cursor.is_over(layout.bounds()) {
                    match delta {
                        mouse::ScrollDelta::Lines { y, .. }
                        | mouse::ScrollDelta::Pixels { y, .. } => {
                            self.value += *y * self.scroll_steps;
                            self.value = self.value.clamp(*self.range.start(), *self.range.end());
                            shell.publish((self.on_change)(self.value));
                        }
                    }
                    return cosmic::iced_core::event::Status::Captured;
                }
            }
            _ => (),
        }

        cosmic::iced_core::event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        tree: &tree::Tree,
        layout: layout::Layout<'_>,
        cursor: cosmic::iced_core::mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> cosmic::iced_core::mouse::Interaction {
        let state = tree.state.downcast_ref::<ColorSliderState>();

        if state.is_dragging {
            mouse::Interaction::Grabbing
        } else if cursor.is_over(layout.bounds()) {
            mouse::Interaction::Grab
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, Message, Theme, Renderer> From<ColorSlider<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Message: Clone + 'a,
{
    fn from(value: ColorSlider<'a, Message>) -> Self {
        Self::new(value)
    }
}

#[derive(Debug, Clone)]
struct ColorSliderState {
    is_dragging: bool,
}
