// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    iced::{gradient::ColorStop, Alignment, Color, Length},
    widget,
};

use crate::{
    colorspace::ColorSpaceMessage as Message, fl, shaders::oklab as shader, widgets::color_slider,
};

const COLOR_STOPS_LIGHTNESS: [ColorStop; 2] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(0.0, 0.0, 0.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(1.0, 1.0, 1.0),
    },
];
const COLOR_STOPS_GREEN_RED: [ColorStop; 2] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(0.0, 1.0, 0.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(1.0, 0.0, 0.0),
    },
];
const COLOR_STOPS_BLUE_YELLOW: [ColorStop; 2] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(0.0, 0.0, 1.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(1.0, 1.0, 0.0),
    },
];

#[derive(Clone)]
pub struct Oklab {
    pub values: [f32; 3],
    pub strings: [String; 3],
}

impl Oklab {
    pub fn from_rgb(rgb: [f32; 3]) -> Self {
        let lab = rgb_to_oklab(rgb[0], rgb[1], rgb[2]);

        Self {
            strings: [lab[0].to_string(), lab[1].to_string(), lab[2].to_string()],
            values: lab,
        }
    }

    pub fn to_rgb(&self) -> [f32; 3] {
        oklab_to_rgb(self.values[0], self.values[1], self.values[2])
    }

    pub fn copy_to_clipboard(&self) -> String {
        format!("{}, {}, {}", self.values[0], self.values[1], self.values[2])
    }
}

impl Oklab {
    pub fn change_value(&mut self, index: usize, value: f32) {
        self.values[index] = value;
        self.strings[index] = value.to_string();
    }

    pub fn change_string(&mut self, index: usize, string: String) {
        if let Ok(value) = string.parse::<f32>() {
            self.values[index] = value;
        }

        self.strings[index] = string;
    }

    pub fn view<'a>(&self, show_graphs: bool) -> cosmic::Element<'a, Message> {
        let values = &self.values;
        let strings = &self.strings;

        let mut lightness = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("lightness")).size(20.0))
                    .push(
                        widget::text_input("", strings[0].clone())
                            .on_input(|string| Message::ChangeString { index: 0, string }),
                    )
                    .align_y(Alignment::Center)
                    .spacing(10.0),
            )
            .push(color_slider(
                0f32..=1f32,
                values[0],
                |value| Message::ChangeValue { index: 0, value },
                &COLOR_STOPS_LIGHTNESS,
            ))
            .spacing(10.0)
            .padding(10.0);
        let mut green_red = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("green-red")).size(20.0))
                    .push(
                        widget::text_input("", strings[1].clone())
                            .on_input(|string| Message::ChangeString { index: 1, string }),
                    )
                    .align_y(Alignment::Center)
                    .spacing(10.0),
            )
            .push(color_slider(
                -0.5..=0.5,
                values[1],
                |value| Message::ChangeValue { index: 1, value },
                &COLOR_STOPS_GREEN_RED,
            ))
            .spacing(10.0)
            .padding(10.0);
        let mut blue_yellow = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("blue-yellow")).size(20.0))
                    .push(
                        widget::text_input("", strings[2].clone())
                            .on_input(|string| Message::ChangeString { index: 2, string }),
                    )
                    .align_y(Alignment::Center)
                    .spacing(10.0),
            )
            .push(color_slider(
                -0.5..=0.5,
                values[2],
                |value| Message::ChangeValue { index: 2, value },
                &COLOR_STOPS_BLUE_YELLOW,
            ))
            .spacing(10.0)
            .padding(10.0);

        if show_graphs {
            lightness = lightness.push(
                cosmic::iced_widget::shader(shader::ColorGraph::<0> {
                    lightness: self.values[0],
                    green_red: self.values[1],
                    blue_yellow: self.values[2],
                })
                .width(Length::Fill),
            );
            green_red = green_red.push(
                cosmic::iced_widget::shader(shader::ColorGraph::<1> {
                    lightness: self.values[0],
                    green_red: self.values[1],
                    blue_yellow: self.values[2],
                })
                .width(Length::Fill),
            );
            blue_yellow = blue_yellow.push(
                cosmic::iced_widget::shader(shader::ColorGraph::<2> {
                    lightness: self.values[0],
                    green_red: self.values[1],
                    blue_yellow: self.values[2],
                })
                .width(Length::Fill),
            );
        }

        let content = widget::column::with_capacity(3)
            .push(widget::container(lightness).class(cosmic::style::Container::Card))
            .push(widget::container(green_red).class(cosmic::style::Container::Card))
            .push(widget::container(blue_yellow).class(cosmic::style::Container::Card))
            .spacing(10.0);

        content.into()
    }
}

// https://bottosson.github.io/posts/oklab/
#[allow(
    clippy::excessive_precision,
    clippy::many_single_char_names,
    clippy::unreadable_literal
)]
pub fn oklab_to_rgb(l: f32, a: f32, b: f32) -> [f32; 3] {
    let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = l - 0.0894841775 * a - 1.2914855480 * b;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    [
        4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
    ]
}

// https://bottosson.github.io/posts/oklab/
#[allow(
    clippy::excessive_precision,
    clippy::many_single_char_names,
    clippy::unreadable_literal
)]
pub fn rgb_to_oklab(r: f32, g: f32, b: f32) -> [f32; 3] {
    let l = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
    let m = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
    let s = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;

    let l_ = l.cbrt();
    let m_ = m.cbrt();
    let s_ = s.cbrt();

    [
        0.2104542553 * l_ + 0.7936177850 * m_ - 0.0040720468 * s_,
        1.9779984951 * l_ - 2.4285922050 * m_ + 0.4505937099 * s_,
        0.0259040371 * l_ + 0.7827717662 * m_ - 0.8086757660 * s_,
    ]
}

#[cfg(test)]
mod test {
    use super::{oklab_to_rgb, rgb_to_oklab};

    #[test]
    fn white() {
        let rgb = [1f32; 3];
        let lab = rgb_to_oklab(rgb[0], rgb[1], rgb[2]);
        assert!(aprox_eq(&lab, &[1.0, 0.0, 0.0]));

        let rgb = oklab_to_rgb(lab[0], lab[1], lab[2]);
        assert!(aprox_eq(&rgb, &[1f32; 3]));
    }

    #[test]
    fn black() {
        let rgb = [0f32; 3];
        let lab = rgb_to_oklab(rgb[0], rgb[1], rgb[2]);
        assert!(aprox_eq(&lab, &[0.0, 0.0, 0.0]));

        let rgb = oklab_to_rgb(lab[0], lab[1], lab[2]);
        assert!(aprox_eq(&rgb, &[0f32; 3]));
    }

    #[test]
    fn red() {
        let rgb = [1f32, 0f32, 0f32];
        let lab = rgb_to_oklab(rgb[0], rgb[1], rgb[2]);
        assert!(aprox_eq(&lab, &[0.6279554, 0.22486305, 0.1258463]));

        let rgb = oklab_to_rgb(lab[0], lab[1], lab[2]);
        assert!(aprox_eq(&rgb, &[1f32, 0f32, 0f32]));
    }

    #[test]
    fn green() {
        let rgb = [0f32, 1f32, 0f32];
        let lab = rgb_to_oklab(rgb[0], rgb[1], rgb[2]);
        assert!(aprox_eq(&lab, &[0.8664396, -0.2338874, 0.1794985]));

        let rgb = oklab_to_rgb(lab[0], lab[1], lab[2]);
        assert!(aprox_eq(&rgb, &[0f32, 1f32, 0f32]));
    }

    #[test]
    fn blue() {
        let rgb = [0f32, 0f32, 1f32];
        let lab = rgb_to_oklab(rgb[0], rgb[1], rgb[2]);
        assert!(aprox_eq(&lab, &[0.4520137, -0.032456964, -0.31152815]));

        let rgb = oklab_to_rgb(lab[0], lab[1], lab[2]);
        assert!(aprox_eq(&rgb, &[0f32, 0f32, 1f32]));
    }

    fn aprox_eq(a: &[f32; 3], b: &[f32; 3]) -> bool {
        const EPSILON: f32 = 1e-4;

        a[0] > b[0] - EPSILON
            && a[0] < b[0] + EPSILON
            && a[1] > b[1] - EPSILON
            && a[1] < b[1] + EPSILON
            && a[2] > b[2] - EPSILON
            && a[2] < b[2] + EPSILON
    }
}
