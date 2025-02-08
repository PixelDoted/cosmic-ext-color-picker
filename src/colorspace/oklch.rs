// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    iced::{gradient::ColorStop, Alignment, Color, Length},
    widget,
};

use crate::{
    colorspace::ColorSpaceMessage as Message, fl, shaders::oklch as shader, widgets::color_slider,
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
const COLOR_STOPS_CHROMA: [ColorStop; 2] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(0.0, 0.0, 0.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(1.0, 0.0, 0.0),
    },
];
const COLOR_STOPS_HUE: [ColorStop; 7] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(0.92, 0.0, 0.22),
    },
    ColorStop {
        offset: 0.0812,
        color: Color::from_rgb(1.0, 0.0, 0.0),
    },
    ColorStop {
        offset: 0.304,
        color: Color::from_rgb(1.0, 1.0, 0.0),
    },
    ColorStop {
        offset: 0.395,
        color: Color::from_rgb(0.0, 1.0, 0.0),
    },
    ColorStop {
        offset: 0.541,
        color: Color::from_rgb(0.0, 1.0, 1.0),
    },
    ColorStop {
        offset: 0.733,
        color: Color::from_rgb(0.0, 0.0, 1.0),
    },
    ColorStop {
        offset: 0.912,
        color: Color::from_rgb(1.0, 0.0, 1.0),
    },
];

#[derive(Clone)]
pub struct Oklch {
    pub values: [f32; 3],
    pub strings: [String; 3],
}

impl Oklch {
    pub fn from_rgb(rgb: [f32; 3]) -> Self {
        let lch = rgb_to_oklch(rgb[0], rgb[1], rgb[2]);

        Self {
            strings: [lch[0].to_string(), lch[1].to_string(), lch[2].to_string()],
            values: lch,
        }
    }

    pub fn to_rgb(&self) -> [f32; 3] {
        oklch_to_rgb(self.values[0], self.values[1], self.values[2])
    }

    pub fn copy_to_clipboard(&self) -> String {
        format!("{}, {}, {}", self.values[0], self.values[1], self.values[2])
    }
}

impl Oklch {
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
                0f32..=1.0f32,
                values[0],
                |value| Message::ChangeValue { index: 0, value },
                &COLOR_STOPS_LIGHTNESS,
            ))
            .spacing(10.0)
            .padding(10.0);
        let mut chroma = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("chroma")).size(20.0))
                    .push(
                        widget::text_input("", strings[1].clone())
                            .on_input(|string| Message::ChangeString { index: 1, string }),
                    )
                    .align_y(Alignment::Center)
                    .spacing(10.0),
            )
            .push(color_slider(
                0f32..=0.37f32,
                values[1],
                |value| Message::ChangeValue { index: 1, value },
                &COLOR_STOPS_CHROMA,
            ))
            .spacing(10.0)
            .padding(10.0);
        let mut hue = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("hue")).size(20.0))
                    .push(
                        widget::text_input("", strings[2].clone())
                            .on_input(|string| Message::ChangeString { index: 2, string }),
                    )
                    .align_y(Alignment::Center)
                    .spacing(10.0),
            )
            .push(color_slider(
                0f32..=360f32,
                values[2],
                |value| Message::ChangeValue { index: 2, value },
                &COLOR_STOPS_HUE,
            ))
            .spacing(10.0)
            .padding(10.0);

        if show_graphs {
            lightness = lightness.push(
                cosmic::iced_widget::shader(shader::ColorGraph::<0> {
                    lightness: self.values[0],
                    chroma: self.values[1],
                    hue: self.values[2],
                })
                .width(Length::Fill),
            );
            chroma = chroma.push(
                cosmic::iced_widget::shader(shader::ColorGraph::<1> {
                    lightness: self.values[0],
                    chroma: self.values[1],
                    hue: self.values[2],
                })
                .width(Length::Fill),
            );
            hue = hue.push(
                cosmic::iced_widget::shader(shader::ColorGraph::<2> {
                    lightness: self.values[0],
                    chroma: self.values[1],
                    hue: self.values[2],
                })
                .width(Length::Fill),
            );
        }

        let content = widget::column::with_capacity(3)
            .push(widget::container(lightness).class(cosmic::style::Container::Card))
            .push(widget::container(chroma).class(cosmic::style::Container::Card))
            .push(widget::container(hue).class(cosmic::style::Container::Card))
            .spacing(10.0);

        content.into()
    }
}

#[allow(clippy::many_single_char_names)]
fn oklch_to_rgb(l: f32, c: f32, h: f32) -> [f32; 3] {
    let h = h.to_radians();
    let a = c * h.cos();
    let b = c * h.sin();

    super::oklab::oklab_to_rgb(l, a, b)
}

#[allow(clippy::many_single_char_names)]
fn rgb_to_oklch(r: f32, g: f32, b: f32) -> [f32; 3] {
    let lab = super::oklab::rgb_to_oklab(r, g, b);
    let mut lch = [
        lab[0],
        (lab[1] * lab[1] + lab[2] * lab[2]).sqrt(),
        lab[2].atan2(lab[1]).to_degrees(),
    ];

    if lch[2] < 0.0 {
        lch[2] += 360.0;
    }

    lch
}

#[cfg(test)]
mod test {
    use super::{oklch_to_rgb, rgb_to_oklch};

    #[test]
    fn white() {
        let rgb = [1f32; 3];
        let lch = rgb_to_oklch(rgb[0], rgb[1], rgb[2]);
        assert!(aprox_eq(&lch, &[1.0, 0.0, 90.0]));

        let rgb = oklch_to_rgb(lch[0], lch[1], lch[2]);
        assert!(aprox_eq(&rgb, &[1f32; 3]));
    }

    #[test]
    fn black() {
        let rgb = [0f32; 3];
        let lch = rgb_to_oklch(rgb[0], rgb[1], rgb[2]);
        assert!(aprox_eq(&lch, &[0.0, 0.0, 0.0]));

        let rgb = oklch_to_rgb(lch[0], lch[1], lch[2]);
        assert!(aprox_eq(&rgb, &[0f32; 3]));
    }

    #[test]
    fn red() {
        let rgb = [1f32, 0f32, 0f32];
        let lch = rgb_to_oklch(rgb[0], rgb[1], rgb[2]);
        assert!(aprox_eq(&lch, &[0.6279554, 0.2576833, 29.233887]));

        let rgb = oklch_to_rgb(lch[0], lch[1], lch[2]);
        assert!(aprox_eq(&rgb, &[1f32, 0f32, 0f32]));
    }

    #[test]
    fn green() {
        let rgb = [0f32, 1f32, 0f32];
        let lch = rgb_to_oklch(rgb[0], rgb[1], rgb[2]);
        assert!(aprox_eq(&lch, &[0.8664396, 0.2948271, 142.49532]));

        let rgb = oklch_to_rgb(lch[0], lch[1], lch[2]);
        assert!(aprox_eq(&rgb, &[0f32, 1f32, 0f32]));
    }

    #[test]
    fn blue() {
        let rgb = [0f32, 0f32, 1f32];
        let lch = rgb_to_oklch(rgb[0], rgb[1], rgb[2]);
        assert!(aprox_eq(&lch, &[0.4520137, 0.31321436, 264.05203]));

        let rgb = oklch_to_rgb(lch[0], lch[1], lch[2]);
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
