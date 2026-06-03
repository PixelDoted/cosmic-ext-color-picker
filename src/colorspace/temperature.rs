// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    iced::{gradient::ColorStop, Alignment, Color},
    widget,
};

use crate::{colorspace::ColorSpaceMessage as Message, fl, widgets::color_slider};

const COLOR_STOPS_KELVIN: [ColorStop; 7] = [
    ColorStop {
        offset: 0.0,
        color: Color::from_rgb(1.0, 0.663, 0.0),
    },
    ColorStop {
        offset: 0.166,
        color: Color::from_rgb(1.0, 1.0, 1.0),
    },
    ColorStop {
        offset: 0.333,
        color: Color::from_rgb(1.0, 1.0, 1.0),
    },
    ColorStop {
        offset: 0.5,
        color: Color::from_rgb(0.842, 0.886, 1.0),
    },
    ColorStop {
        offset: 0.666,
        color: Color::from_rgb(0.768, 0.841, 1.0),
    },
    ColorStop {
        offset: 0.833,
        color: Color::from_rgb(0.727, 0.815, 1.0),
    },
    ColorStop {
        offset: 1.0,
        color: Color::from_rgb(0.7, 0.798, 1.0),
    },
];

#[derive(Clone)]
pub struct Temperature {
    pub values: [f32; 1],
    pub strings: [String; 1],
}

impl Temperature {
    pub fn from_rgb(rgb: [f32; 3]) -> Self {
        let kelvin = rgb_to_kelvin(rgb[0], rgb[1], rgb[2]);

        Self {
            strings: [kelvin[0].to_string()],
            values: kelvin,
        }
    }

    pub fn to_rgb(&self) -> [f32; 3] {
        kelvin_to_rgb(self.values[0])
    }

    pub fn copy_to_clipboard(&self) -> String {
        format!("{}K", self.values[0])
    }
}

impl Temperature {
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

        let red = widget::column::with_capacity(3)
            .push(
                widget::row::with_capacity(2)
                    .push(widget::text(fl!("kelvin")).size(20.0))
                    .push(
                        widget::text_input("", strings[0].clone())
                            .on_input(|string| Message::ChangeString { index: 0, string }),
                    )
                    .align_y(Alignment::Center)
                    .spacing(10.0),
            )
            .push(color_slider(
                1000f32..=16000f32,
                values[0],
                |value| Message::ChangeValue { index: 0, value },
                &COLOR_STOPS_KELVIN,
            ))
            .spacing(10.0)
            .padding(10.0);

        let content = widget::column::with_capacity(3)
            .push(widget::container(red).class(cosmic::style::Container::Card))
            .spacing(10.0);

        content.into()
    }
}

// http://www.tannerhelland.com/4435/convert-temperature-rgb-algorithm-code/
#[allow(clippy::many_single_char_names)]
fn kelvin_to_rgb(k: f32) -> [f32; 3] {
    let k: f64 = k as f64 / 100.0;

    let r: f64;
    let g: f64;
    let b: f64;

    if k <= 66.0 {
        r = 255.0;
        g = 99.4708025861 * k.log2() - 161.1195681661;

        if k <= 19.0 {
            b = 0.0;
        } else {
            b = 138.5177312231 * (k - 10.0).log2() - 305.0447927307;
        }
    } else {
        r = 329.698727446 * (k - 60.0).powf(-0.1332047592);
        g = 288.1221695283 * (k - 60.0).powf(-0.0755148492);
        b = 255.0;
    }

    [(r / 255.0) as f32, (g / 255.0) as f32, (b / 255.0) as f32]
}

#[allow(clippy::many_single_char_names, clippy::float_cmp)]
fn rgb_to_kelvin(r: f32, g: f32, b: f32) -> [f32; 1] {
    const MAPPING: [[f32; 4]; 40] = [
        [1E3, 1E0, 6.639815E-1, 0E0],
        [1.3846154E3, 1E0, 8.4711903E-1, 0E0],
        [1.7692307E3, 1E0, 9.850663E-1, 0E0],
        [2.1538462E3, 1E0, 1.0957688E0, 7.203854E-1],
        [2.5384617E3, 1E0, 1.1882334E0, 9.458366E-1],
        [2.9230771E3, 1E0, 1.2676281E0, 1.1207101E0],
        [3.3076926E3, 1E0, 1.3371941E0, 1.2635922E0],
        [3.692308E3, 1E0, 1.3990992E0, 1.3843973E0],
        [4.0769236E3, 1E0, 1.4548645E0, 1.4890435E0],
        [4.461539E3, 1E0, 1.5055987E0, 1.5813478E0],
        [4.8461543E3, 1E0, 1.552135E0, 1.663917E0],
        [5.2307695E3, 1E0, 1.5951153E0, 1.7386097E0],
        [5.615385E3, 1E0, 1.6350447E0, 1.8067989E0],
        [6E3, 1E0, 1.6723278E0, 1.8695269E0],
        [6.384615E3, 1E0, 1.7072936E0, 1.927604E0],
        [6.7692305E3, 9.852584E-1, 9.6855897E-1, 1E0],
        [7.1538457E3, 9.3345624E-1, 9.3935245E-1, 1E0],
        [7.538461E3, 8.983623E-1, 9.191658E-1, 1E0],
        [7.923076E3, 8.7205255E-1, 9.03807E-1, 1E0],
        [8.307691E3, 8.511289E-1, 8.914487E-1, 1E0],
        [8.692307E3, 8.3383036E-1, 8.8113177E-1, 1E0],
        [9.076922E3, 8.191301E-1, 8.7229145E-1, 1E0],
        [9.461537E3, 8.0637884E-1, 8.6456734E-1, 1E0],
        [9.846152E3, 7.9514074E-1, 8.577159E-1, 1E0],
        [1.0230768E4, 7.8510964E-1, 8.5156476E-1, 1E0],
        [1.0615383E4, 7.760625E-1, 8.459878E-1, 1E0],
        [1.0999998E4, 7.6783204E-1, 8.4088975E-1, 1E0],
        [1.1384613E4, 7.602896E-1, 8.36197E-1, 1E0],
        [1.1769229E4, 7.5333446E-1, 8.318518E-1, 1E0],
        [1.2153844E4, 7.468859E-1, 8.2780755E-1, 1E0],
        [1.2538459E4, 7.408787E-1, 8.240264E-1, 1E0],
        [1.2923074E4, 7.352593E-1, 8.2047737E-1, 1E0],
        [1.3307689E4, 7.29983E-1, 8.1713426E-1, 1E0],
        [1.3692305E4, 7.250123E-1, 8.139753E-1, 1E0],
        [1.407692E4, 7.203157E-1, 8.109818E-1, 1E0],
        [1.4461535E4, 7.158659E-1, 8.081379E-1, 1E0],
        [1.484615E4, 7.1163964E-1, 8.054297E-1, 1E0],
        [1.5230766E4, 7.076167E-1, 8.028453E-1, 1E0],
        [1.5615381E4, 7.0377934E-1, 8.003742E-1, 1E0],
        [1.5999996E4, 7.001121E-1, 7.980072E-1, 1E0],
    ];

    // Find closest
    let mut closest = [usize::MAX, usize::MAX];
    let mut closest_dist = [f32::INFINITY, f32::INFINITY];
    for i in 0..MAPPING.len() {
        let arr = MAPPING[i];
        let dist = (r - arr[1]).powf(2.0) + (g - arr[2]).powf(2.0) + (b - arr[3]).powf(2.0);

        if dist < closest_dist[0] {
            closest[1] = closest[0];
            closest_dist[1] = closest_dist[0];
            closest[0] = i;
            closest_dist[0] = dist;
        }
    }

    // Interpolate
    if closest[1] == usize::MAX {
        [MAPPING[closest[0]][0]]
    } else {
        let a = MAPPING[closest[0]];
        let b = MAPPING[closest[1]];

        let t = closest_dist[0] / closest_dist[1];
        let k = (1.0 - t) * a[0] + t * b[0];
        [k]
    }
}
