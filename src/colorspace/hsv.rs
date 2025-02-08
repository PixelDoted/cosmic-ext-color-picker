// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    iced::{gradient::ColorStop, Alignment, Color},
    widget,
};

use crate::{
    colorspace::ColorSpaceMessage as Message, fl, shaders::hsv as shader, widgets::color_slider,
};

const COLOR_STOPS_HUE: [ColorStop; 7] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(1.0, 0.0, 0.0),
    },
    ColorStop {
        offset: 0.166,
        color: Color::from_rgb(1.0, 1.0, 0.0),
    },
    ColorStop {
        offset: 0.333,
        color: Color::from_rgb(0.0, 1.0, 0.0),
    },
    ColorStop {
        offset: 0.5,
        color: Color::from_rgb(0.0, 1.0, 1.0),
    },
    ColorStop {
        offset: 0.666,
        color: Color::from_rgb(0.0, 0.0, 1.0),
    },
    ColorStop {
        offset: 0.833,
        color: Color::from_rgb(1.0, 0.0, 1.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(1.0, 0.0, 0.0),
    },
];
const COLOR_STOPS_SATURATION: [ColorStop; 2] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(0.0, 0.0, 0.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(1.0, 0.0, 0.0),
    },
];
const COLOR_STOPS_VALUE: [ColorStop; 2] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(0.0, 0.0, 0.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(1.0, 1.0, 1.0),
    },
];

#[derive(Clone)]
pub struct Hsv {
    pub values: [f32; 3],
    pub strings: [String; 3],
}

impl Hsv {
    pub fn from_rgb(rgb: [f32; 3]) -> Self {
        let hsv = rgb_to_hsv(rgb[0], rgb[1], rgb[2]);

        Self {
            strings: [hsv[0].to_string(), hsv[1].to_string(), hsv[2].to_string()],
            values: hsv,
        }
    }

    pub fn to_rgb(&self) -> [f32; 3] {
        hsv_to_rgb(self.values[0], self.values[1], self.values[2])
    }

    pub fn copy_to_clipboard(&self) -> String {
        format!("{}, {}, {}", self.values[0], self.values[1], self.values[2])
    }
}

impl Hsv {
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

        let red = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("hue")).size(20.0))
                    .push(
                        widget::text_input("", strings[0].clone())
                            .on_input(|string| Message::ChangeString { index: 0, string }),
                    )
                    .align_y(Alignment::Center)
                    .spacing(10.0),
            )
            .push(color_slider(
                0f32..=360f32,
                values[0],
                |value| Message::ChangeValue { index: 0, value },
                &COLOR_STOPS_HUE,
            ))
            .spacing(10.0)
            .padding(10.0);
        let green = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("saturation")).size(20.0))
                    .push(
                        widget::text_input("", strings[1].clone())
                            .on_input(|string| Message::ChangeString { index: 1, string }),
                    )
                    .align_y(Alignment::Center)
                    .spacing(10.0),
            )
            .push(color_slider(
                0f32..=1f32,
                values[1],
                |value| Message::ChangeValue { index: 1, value },
                &COLOR_STOPS_SATURATION,
            ))
            .spacing(10.0)
            .padding(10.0);
        let blue = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("value")).size(20.0))
                    .push(
                        widget::text_input("", strings[2].clone())
                            .on_input(|string| Message::ChangeString { index: 2, string }),
                    )
                    .align_y(Alignment::Center)
                    .spacing(10.0),
            )
            .push(color_slider(
                0f32..=1f32,
                values[2],
                |value| Message::ChangeValue { index: 2, value },
                &COLOR_STOPS_VALUE,
            ))
            .spacing(10.0)
            .padding(10.0);

        let mut content = widget::column::with_capacity(3)
            .push(widget::container(red).class(cosmic::style::Container::Card))
            .push(widget::container(green).class(cosmic::style::Container::Card))
            .push(widget::container(blue).class(cosmic::style::Container::Card))
            .spacing(10.0);

        if show_graphs {
            content = content.push(
                widget::container(
                    widget::container(
                        cosmic::iced_widget::shader(shader::ColorGraph {
                            hue: self.values[0],
                            saturation: self.values[1],
                            value: self.values[2],
                        })
                        .width(100)
                        .height(100),
                    )
                    .padding(10.0),
                )
                .class(cosmic::style::Container::Card),
            );
        }

        content.into()
    }
}

#[allow(clippy::many_single_char_names)]
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [f32; 3] {
    let c = v * s;
    let h_ = h / 60.0;
    let x = c * (1.0 - (h_ % 2.0 - 1.0).abs());

    let (r1, g1, b1) = if (0.0..1.0).contains(&h_) {
        (c, x, 0.0)
    } else if (1.0..2.0).contains(&h_) {
        (x, c, 0.0)
    } else if (2.0..3.0).contains(&h_) {
        (0.0, c, x)
    } else if (3.0..4.0).contains(&h_) {
        (0.0, x, c)
    } else if (4.0..5.0).contains(&h_) {
        (x, 0.0, c)
    } else {
        // otherwise (5.0 <= h' < 6.0)
        (c, 0.0, x)
    };

    let m = v - c;
    [r1 + m, g1 + m, b1 + m]
}

#[allow(clippy::many_single_char_names, clippy::float_cmp)]
fn rgb_to_hsv(r: f32, g: f32, b: f32) -> [f32; 3] {
    let x_max = r.max(g).max(b);
    let x_min = r.min(g).min(b);
    let c = x_max - x_min;
    let mut h = if c == 0.0 {
        0.0
    } else if x_max == r {
        60.0 * ((g - b) / c % 6.0)
    } else if x_max == g {
        60.0 * ((b - r) / c + 2.0)
    } else if x_max == b {
        60.0 * ((r - g) / c + 4.0)
    } else {
        // Default to (c = 0)
        0.0
    };

    if h < 0.0 {
        h += 360.0;
    }

    let s = if x_max == 0.0 { 0.0 } else { c / x_max };

    [h, s, x_max]
}

// ---- Tests ----
#[cfg(test)]
mod test {
    use super::{hsv_to_rgb, rgb_to_hsv};

    #[test]
    fn white() {
        let rgb = [1f32; 3];
        let hsv = rgb_to_hsv(rgb[0], rgb[1], rgb[2]);
        assert!(aprox_eq(&hsv, &[0.0, 0.0, 1.0]));

        let rgb = hsv_to_rgb(hsv[0], hsv[1], hsv[2]);
        assert!(aprox_eq(&rgb, &[1f32; 3]));
    }

    #[test]
    fn black() {
        let rgb = [0f32; 3];
        let hsv = rgb_to_hsv(rgb[0], rgb[1], rgb[2]);
        assert!(aprox_eq(&hsv, &[0.0, 0.0, 0.0]));

        let rgb = hsv_to_rgb(hsv[0], hsv[1], hsv[2]);
        assert!(aprox_eq(&rgb, &[0f32; 3]));
    }

    #[test]
    fn red() {
        let rgb = [1f32, 0f32, 0f32];
        let hsv = rgb_to_hsv(rgb[0], rgb[1], rgb[2]);
        assert!(aprox_eq(&hsv, &[0.0, 1.0, 1.0]));

        let rgb = hsv_to_rgb(hsv[0], hsv[1], hsv[2]);
        assert!(aprox_eq(&rgb, &[1f32, 0f32, 0f32]));
    }

    #[test]
    fn green() {
        let rgb = [0f32, 1f32, 0f32];
        let hsv = rgb_to_hsv(rgb[0], rgb[1], rgb[2]);
        assert!(aprox_eq(&hsv, &[120.0, 1.0, 1.0]));

        let rgb = hsv_to_rgb(hsv[0], hsv[1], hsv[2]);
        assert!(aprox_eq(&rgb, &[0.0, 1.0, 0.0]));
    }

    #[test]
    fn blue() {
        let rgb = [0f32, 0f32, 1f32];
        let hsv = rgb_to_hsv(rgb[0], rgb[1], rgb[2]);
        assert!(aprox_eq(&hsv, &[240.0, 1.0, 1.0]));

        let rgb = hsv_to_rgb(hsv[0], hsv[1], hsv[2]);
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
