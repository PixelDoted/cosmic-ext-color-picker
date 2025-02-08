// SPDX-License-Identifier: GPL-3.0-only

use std::collections::HashMap;

use crate::colorspace::{ColorSpace, ColorSpaceCombo, ColorSpaceMessage};
use crate::fl;
use crate::widgets::color_block;
use cosmic::app::context_drawer::ContextDrawer;
use cosmic::app::{Core, Task};
use cosmic::iced::alignment::{Horizontal, Vertical};
use cosmic::iced::keyboard::{Key, Modifiers};
use cosmic::iced::{clipboard, Length};
use cosmic::iced::{event, keyboard::Event as KeyEvent, Color, Event, Subscription};
use cosmic::iced_widget::scrollable::{Direction, Scrollbar};
use cosmic::widget::menu::{self, action::MenuAction, MenuBar};
use cosmic::{theme, widget, Application, ApplicationExt, Apply, Element};
use log::info;

pub struct ColorPicker {
    pub spaces: Vec<ColorSpace>,
    last_edited: usize,
    show_graphs: bool,
    expanded: bool,

    colorspace_selections: Vec<ColorSpaceCombo>,
    colorspace_names: Vec<String>,
    keybinds: HashMap<menu::KeyBind, Action>,
    core: Core,
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    ColorSpace {
        index: usize,
        message: ColorSpaceMessage,
    },
    ChangeColorSpace {
        index: usize,
        selected: ColorSpaceCombo,
    },
    AddSpace,
    RemoveSpace(usize),

    ToggleGraphs,
    ToggleExpanded,
    ToggleAboutPage,
    LaunchUrl(String),

    CopyToClipboard(usize),
    PickScreenRequest(usize),
    PickScreenResponse((usize, ashpd::desktop::Color)),
    Key(Key, Modifiers),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    ToggleGraphs,
    ToggleExpanded,
    About,
}

impl MenuAction for Action {
    type Message = Message;

    fn message(&self) -> Message {
        match self {
            Action::ToggleGraphs => Message::ToggleGraphs,
            Action::ToggleExpanded => Message::ToggleExpanded,
            Action::About => Message::ToggleAboutPage,
        }
    }
}

impl Application for ColorPicker {
    type Executor = cosmic::executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "io.github.pixeldoted.cosmic-ext-color-picker";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn header_start(&self) -> Vec<Element<Self::Message>> {
        vec![MenuBar::new(vec![menu::Tree::with_children(
            menu::root(fl!("view")),
            menu::items(
                &self.keybinds,
                vec![
                    menu::Item::CheckBox(
                        fl!("graphs"),
                        None,
                        self.show_graphs,
                        Action::ToggleGraphs,
                    ),
                    menu::Item::CheckBox(
                        fl!("expanded"),
                        None,
                        self.expanded,
                        Action::ToggleExpanded,
                    ),
                    menu::Item::Button(fl!("menu-about"), None, Action::About),
                ],
            ),
        )])
        .into()]
    }

    fn header_center(&self) -> Vec<Element<Self::Message>> {
        vec![widget::text::heading(fl!("app-title")).into()]
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let mut keybinds = HashMap::new();
        keybinds.insert(
            menu::KeyBind {
                modifiers: vec![menu::key_bind::Modifier::Ctrl],
                key: Key::Character("g".into()),
            },
            Action::ToggleGraphs,
        );
        keybinds.insert(
            menu::KeyBind {
                modifiers: vec![menu::key_bind::Modifier::Ctrl],
                key: Key::Character("e".into()),
            },
            Action::ToggleExpanded,
        );

        let mut app = ColorPicker {
            spaces: vec![ColorSpace::default()],
            last_edited: 0,
            show_graphs: false,
            expanded: false,

            colorspace_selections: vec![
                ColorSpaceCombo::Rgb,
                ColorSpaceCombo::Hsv,
                ColorSpaceCombo::Oklab,
                ColorSpaceCombo::Oklch,
                ColorSpaceCombo::Cmyk,
            ],
            colorspace_names: vec![],
            keybinds,
            core,
        };

        app.colorspace_names = app
            .colorspace_selections
            .iter()
            .map(ToString::to_string)
            .collect();

        let command = app.set_window_title(fl!("app-title"));
        (app, command)
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::None => (),
            Message::ColorSpace { index: i, message } => match message {
                ColorSpaceMessage::ChangeValue { index, value } => {
                    self.spaces[i].change_value(index, value);
                }
                ColorSpaceMessage::ChangeString { index, string } => {
                    self.spaces[i].change_string(index, string);
                }
            },
            Message::ChangeColorSpace { index, selected } => {
                self.spaces[index] = match selected {
                    ColorSpaceCombo::Rgb => self.spaces[index].to_rgb(),
                    ColorSpaceCombo::Hsv => self.spaces[index].to_hsv(),
                    ColorSpaceCombo::Oklab => self.spaces[index].to_oklab(),
                    ColorSpaceCombo::Oklch => self.spaces[index].to_oklch(),
                    ColorSpaceCombo::Cmyk => self.spaces[index].to_cmyk(),
                };
            }
            Message::AddSpace => {
                self.spaces.push(ColorSpace::default());
            }
            Message::RemoveSpace(index) => {
                self.spaces.remove(index);
            }

            Message::ToggleGraphs => {
                self.show_graphs = !self.show_graphs;
            }
            Message::ToggleExpanded => {
                self.expanded = !self.expanded;
            }
            Message::ToggleAboutPage => {
                self.core.window.show_context = !self.core.window.show_context;
            }
            Message::LaunchUrl(url) => match open::that_detached(&url) {
                Ok(()) => {}
                Err(e) => {
                    log::warn!("Failed to open {:?}: {}", url, e);
                }
            },

            Message::CopyToClipboard(index) => {
                return self.copy_to_clipboard(index);
            }
            Message::PickScreenRequest(index) => {
                return cosmic::task::future(async move {
                    let req = ashpd::desktop::Color::pick().send().await;
                    let Ok(req) = req else {
                        println!("{req:?}");
                        return Message::None;
                    };

                    let result = req.response();
                    let Ok(color) = result else {
                        println!("{result:?}");
                        return Message::None;
                    };

                    println!("{index} {color:?}");

                    Message::PickScreenResponse((index, color))
                });
            }
            Message::PickScreenResponse((index, color)) => {
                let (r, g, b) = (color.red(), color.green(), color.blue());

                #[allow(clippy::cast_possible_truncation)]
                self.spaces[index].convert_from_rgb([r as f32, g as f32, b as f32]);
            }
            Message::Key(key, modifiers) => {
                for (key_bind, action) in &self.keybinds {
                    if key_bind.matches(modifiers, &key) {
                        return self.update(action.message());
                    }
                }

                if modifiers.control() && key == Key::Character("c".into()) {
                    return self.copy_to_clipboard(self.last_edited);
                }
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let mut contents = widget::row::with_capacity(self.spaces.len());

        for (colorspace, index) in self.spaces.iter().zip(0..) {
            let (rgb, content, combo_selection) = match colorspace {
                ColorSpace::Rgb(rgb) => (
                    rgb.to_rgb(),
                    rgb.view(self.show_graphs),
                    0, //ColorSpaceCombo::Rgb,
                ),
                ColorSpace::Hsv(hsv) => (
                    hsv.to_rgb(),
                    hsv.view(self.show_graphs),
                    1, //ColorSpaceCombo::Hsv,
                ),
                ColorSpace::Oklab(oklab) => (
                    oklab.to_rgb(),
                    oklab.view(self.show_graphs),
                    2, //ColorSpaceCombo::Oklab,
                ),
                ColorSpace::Oklch(oklch) => (
                    oklch.to_rgb(),
                    oklch.view(self.show_graphs),
                    3, //ColorSpaceCombo::Oklch,
                ),
                ColorSpace::Cmyk(cmyk) => (
                    cmyk.to_rgb(),
                    cmyk.view(self.show_graphs),
                    4, //ColorSpaceCombo::Cmyk,
                ),
            };

            let min_rgb = rgb[0].min(rgb[1]).min(rgb[2]).min(0.0);
            let max_rgb = rgb[0].max(rgb[1]).max(rgb[2]).max(1.0) - min_rgb;
            let norm_rgb = [
                (rgb[0] - min_rgb) / max_rgb,
                (rgb[1] - min_rgb) / max_rgb,
                (rgb[2] - min_rgb) / max_rgb,
            ];

            let mut sidebar = widget::column::with_capacity(3)
                .push(
                    widget::row::with_capacity(2)
                        .push(
                            color_block(Color::from_rgb(rgb[0], rgb[1], rgb[2]))
                                .border([true, false, false, true])
                                .height(100.0),
                        )
                        .push(
                            color_block(Color::from_rgb(norm_rgb[0], norm_rgb[1], norm_rgb[2]))
                                .border([false, true, true, false])
                                .height(100.0),
                        ),
                )
                .push(
                    widget::row::with_capacity(3)
                        .push(
                            widget::button::icon(widget::icon::from_name("edit-copy-symbolic"))
                                .on_press(Message::CopyToClipboard(index))
                                .tooltip("Copy to Clipboard"),
                        )
                        .push(
                            widget::button::icon(widget::icon::from_name("edit-find-symbolic"))
                                .on_press(Message::PickScreenRequest(index))
                                .tooltip("Pick a color from the screen"),
                        )
                        .push(widget::Space::with_width(Length::Fill))
                        .push(
                            widget::button::icon(widget::icon::from_name(
                                "user-trash-full-symbolic",
                            ))
                            .on_press(Message::RemoveSpace(index))
                            .class(theme::Button::Destructive)
                            .tooltip("Delete"),
                        ),
                )
                .push(
                    widget::dropdown(&self.colorspace_names, Some(combo_selection), move |t| {
                        Message::ChangeColorSpace {
                            index,
                            selected: self.colorspace_selections[t].clone(),
                        }
                    })
                    .width(Length::Fill),
                )
                .spacing(10.0);

            if self.expanded {
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let srgb = [
                    (norm_rgb[0] * 255.0) as u8,
                    (norm_rgb[1] * 255.0) as u8,
                    (norm_rgb[2] * 255.0) as u8,
                ];
                let srgb_text = format!("{}, {}, {}", srgb[0], srgb[1], srgb[2]);
                let hex_text = format!("#{}", hex::encode(srgb));

                let col = widget::ListColumn::new()
                    .add(
                        widget::text_input("0, 0, 0", srgb_text)
                            .on_input(|_| Message::None)
                            .label("sRGB"),
                    )
                    .add(
                        widget::text_input("#000000", hex_text)
                            .on_input(|_| Message::None)
                            .label("Hex"),
                    );

                sidebar = sidebar.push(col);
            }

            let sidebar_container = widget::Container::new(sidebar)
                .class(theme::Container::Card)
                .padding(10.0);

            let elem: Element<'_, Message> = if self.expanded {
                widget::row::with_capacity(2)
                    .push(sidebar_container)
                    .push(content.map(move |message| Message::ColorSpace { index, message }))
                    .spacing(10.0)
                    .padding(10.0)
                    .width(590.0)
                    .into()
            } else {
                widget::column::with_capacity(2)
                    .push(sidebar_container)
                    .push(content.map(move |message| Message::ColorSpace { index, message }))
                    .spacing(10.0)
                    .padding(10.0)
                    .width(300.0)
                    .into()
            };

            contents = contents.push(widget::container(elem.apply(widget::scrollable)));
        }

        {
            contents = contents.push(
                widget::container(
                    widget::button::icon(widget::icon::from_name("list-add-symbolic"))
                        .icon_size(32)
                        .on_press(Message::AddSpace),
                )
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .width(50.0)
                .height(200.0),
            );
        }

        widget::scrollable(contents)
            .direction(Direction::Horizontal(Scrollbar::new()))
            .height(Length::Fill)
            .into()
    }

    fn context_drawer(&self) -> Option<ContextDrawer<Self::Message>> {
        if !self.core.window.show_context {
            return None;
        }

        Some(Self::about())
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        Subscription::batch(vec![event::listen_with(
            |event, status, _windowid| match event {
                Event::Keyboard(KeyEvent::KeyPressed { key, modifiers, .. }) => match status {
                    event::Status::Ignored => Some(Message::Key(key, modifiers)),
                    event::Status::Captured => None,
                },
                _ => None,
            },
        )])
    }
}

impl ColorPicker {
    fn copy_to_clipboard(&self, index: usize) -> Task<Message> {
        let contents = match &self.spaces[index] {
            ColorSpace::Rgb(rgb) => rgb.copy_to_clipboard(),
            ColorSpace::Hsv(hsv) => hsv.copy_to_clipboard(),
            ColorSpace::Oklab(oklab) => oklab.copy_to_clipboard(),
            ColorSpace::Oklch(oklch) => oklch.copy_to_clipboard(),
            ColorSpace::Cmyk(cmyk) => cmyk.copy_to_clipboard(),
        };

        info!("Copying \"{}\" to clipboard", contents);
        clipboard::write(contents)
    }

    fn about<'a>() -> ContextDrawer<'a, Message> {
        let repository = "https://github.com/PixelDoted/cosmic-ext-color-picker";
        let hash = env!("VERGEN_GIT_SHA");
        let short_hash = &hash[0..7];
        let date = env!("VERGEN_GIT_COMMIT_DATE");

        let content = widget::column::with_capacity(4)
            .push(widget::svg(widget::svg::Handle::from_memory(
                &include_bytes!(
                    "../res/icons/hicolor/scalable/apps/io.github.pixeldoted.cosmic-ext-color-picker.svg"
                )[..],
            )))
            .push(widget::text::title3(fl!("app-title")))
            .push(
                widget::button::link(repository)
                    .on_press(Message::LaunchUrl(repository.to_string()))
                    .padding(0),
            )
            .push(
                widget::button::link(fl!("git-description", hash = short_hash, date = date))
                    .on_press(Message::LaunchUrl(format!(
                        "{repository}/commits/{hash}"
                    )))
                    .padding(0),
            )
            .into();

        ContextDrawer {
            title: Some("About".into()),
            header_actions: vec![],
            header: None,
            content,
            footer: None,
            on_close: Message::ToggleAboutPage,
        }
    }
}
