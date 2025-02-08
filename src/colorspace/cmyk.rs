// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    iced::{gradient::ColorStop, Alignment, Color},
    widget,
};

use crate::{colorspace::ColorSpaceMessage as Message, fl, widgets::color_slider};

const COLOR_STOPS_CYAN: [ColorStop; 2] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(1.0, 1.0, 1.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(0.0, 1.0, 1.0),
    },
];
const COLOR_STOPS_MAGENTA: [ColorStop; 2] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(1.0, 1.0, 1.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(1.0, 0.0, 1.0),
    },
];
const COLOR_STOPS_YELLOW: [ColorStop; 2] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(1.0, 1.0, 1.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(1.0, 1.0, 0.0),
    },
];
const COLOR_STOPS_BLACK: [ColorStop; 2] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(1.0, 1.0, 1.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(0.0, 0.0, 0.0),
    },
];

#[derive(Clone)]
pub struct Cmyk {
    pub values: [f32; 4],
    pub strings: [String; 4],
}

impl Default for Cmyk {
    fn default() -> Self {
        Self {
            values: [0.0; 4],
            strings: ["0".into(), "0".into(), "0".into(), "0".into()],
        }
    }
}

impl Cmyk {
    pub fn from_rgb(rgb: [f32; 3]) -> Self {
        let cmyk = rgb_to_cmyk(rgb[0], rgb[1], rgb[2]);

        Self {
            strings: [
                cmyk[0].to_string(),
                cmyk[1].to_string(),
                cmyk[2].to_string(),
                cmyk[3].to_string(),
            ],
            values: cmyk,
        }
    }

    pub fn to_rgb(&self) -> [f32; 3] {
        cmyk_to_rgb(
            self.values[0],
            self.values[1],
            self.values[2],
            self.values[3],
        )
    }

    pub fn copy_to_clipboard(&self) -> String {
        format!(
            "{}, {}, {}, {}",
            self.values[0], self.values[1], self.values[2], self.values[3]
        )
    }
}

impl Cmyk {
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

    pub fn view<'a>(&self, _show_graphs: bool) -> cosmic::Element<'a, Message> {
        let values = &self.values;
        let strings = &self.strings;

        let cyan = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("cyan")).size(20.0))
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
                &COLOR_STOPS_CYAN,
            ))
            .spacing(10.0)
            .padding(10.0);
        let magenta = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("magenta")).size(20.0))
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
                &COLOR_STOPS_MAGENTA,
            ))
            .spacing(10.0)
            .padding(10.0);
        let yellow = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("yellow")).size(20.0))
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
                &COLOR_STOPS_YELLOW,
            ))
            .spacing(10.0)
            .padding(10.0);
        let black = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("black")).size(20.0))
                    .push(
                        widget::text_input("", strings[3].clone())
                            .on_input(|string| Message::ChangeString { index: 3, string }),
                    )
                    .align_y(Alignment::Center)
                    .spacing(10.0),
            )
            .push(color_slider(
                0f32..=1f32,
                values[3],
                |value| Message::ChangeValue { index: 3, value },
                &COLOR_STOPS_BLACK,
            ))
            .spacing(10.0)
            .padding(10.0);

        let content = widget::column::with_capacity(3)
            .push(widget::container(cyan).class(cosmic::style::Container::Card))
            .push(widget::container(magenta).class(cosmic::style::Container::Card))
            .push(widget::container(yellow).class(cosmic::style::Container::Card))
            .push(widget::container(black).class(cosmic::style::Container::Card))
            .spacing(10.0);

        content.into()
    }
}

#[allow(clippy::many_single_char_names)]
fn cmyk_to_rgb(c: f32, m: f32, y: f32, k: f32) -> [f32; 3] {
    let inv_k = 1.0 - k;
    let r = (1.0 - c + k) * inv_k;
    let g = (1.0 - m + k) * inv_k;
    let b = (1.0 - y + k) * inv_k;

    [r, g, b]
}

#[allow(clippy::many_single_char_names)]
fn rgb_to_cmyk(r: f32, g: f32, b: f32) -> [f32; 4] {
    let inv_k = r.max(g).max(b);
    if inv_k <= f32::EPSILON {
        return [0.0, 0.0, 0.0, 1.0];
    }

    let k = 1.0 - inv_k;
    let c = (1.0 - r - k) / inv_k;
    let m = (1.0 - g - k) / inv_k;
    let y = (1.0 - b - k) / inv_k;

    [c, m, y, k]
}

#[cfg(test)]
mod test {
    use super::{cmyk_to_rgb, rgb_to_cmyk};

    #[test]
    fn white() {
        let rgb = [1f32; 3];
        let cmyk = rgb_to_cmyk(rgb[0], rgb[1], rgb[2]);
        assert!(aprox_eq(&cmyk, &[0.0; 4]));

        let rgb = cmyk_to_rgb(cmyk[0], cmyk[1], cmyk[2], cmyk[3]);
        assert!(aprox_eq(&rgb, &[1f32; 3]));
    }

    #[test]
    fn black() {
        let rgb = [0f32; 3];
        let cmyk = rgb_to_cmyk(rgb[0], rgb[1], rgb[2]);
        assert!(aprox_eq(&cmyk, &[0.0, 0.0, 0.0, 1.0]));

        let rgb = cmyk_to_rgb(cmyk[0], cmyk[1], cmyk[2], cmyk[3]);
        assert!(aprox_eq(&rgb, &[0f32; 3]));
    }

    #[test]
    fn red() {
        let rgb = [1f32, 0f32, 0f32];
        let cmyk = rgb_to_cmyk(rgb[0], rgb[1], rgb[2]);
        assert!(aprox_eq(&cmyk, &[0.0, 1.0, 1.0, 0.0]));

        let rgb = cmyk_to_rgb(cmyk[0], cmyk[1], cmyk[2], cmyk[3]);
        assert!(aprox_eq(&rgb, &[1f32, 0f32, 0f32]));
    }

    #[test]
    fn green() {
        let rgb = [0f32, 1f32, 0f32];
        let cmyk = rgb_to_cmyk(rgb[0], rgb[1], rgb[2]);
        assert!(aprox_eq(&cmyk, &[1.0, 0.0, 1.0, 0.0]));

        let rgb = cmyk_to_rgb(cmyk[0], cmyk[1], cmyk[2], cmyk[3]);
        assert!(aprox_eq(&rgb, &[0.0, 1.0, 0.0]));
    }

    #[test]
    fn blue() {
        let rgb = [0f32, 0f32, 1f32];
        let cmyk = rgb_to_cmyk(rgb[0], rgb[1], rgb[2]);
        assert!(aprox_eq(&cmyk, &[1.0, 1.0, 0.0, 0.0]));

        let rgb = cmyk_to_rgb(cmyk[0], cmyk[1], cmyk[2], cmyk[3]);
        assert!(aprox_eq(&rgb, &[0f32, 0f32, 1f32]));
    }

    fn aprox_eq(a: &[f32], b: &[f32]) -> bool {
        const EPSILON: f32 = 1e-4;

        a.iter()
            .zip(b)
            .all(|(a, b)| *a > *b - EPSILON && *a < *b + EPSILON)
    }
}
