use std::os::macos::fs::MetadataExt;
use std::path::Path;

use iced::alignment::Vertical;
use iced::widget::scrollable::{Scrollbar, Scroller};
use iced::widget::{
    button, text, text_input, Button, Column, Container, Row, Scrollable, TextInput
};
use iced::{Border, Color, Element, Length, Shadow};

use crate::frontend::message::Message;

use crate::backend::util::format_raw_size;

struct OxidriveColour;
impl OxidriveColour {
    fn new(r: u8, g: u8, b: u8) -> Color { Color::from_rgb8(r, g, b) }
    fn hex(hex: &str) -> Color { Color::parse(hex).unwrap() }

    fn background()     -> Color { Self::hex("#1f2335") }
    fn foreground()     -> Color { Self::hex("#24283b") }
    fn accent()         -> Color { Self::hex("#292e42") }
    fn colour()         -> Color { Self::hex("#9d7cd8") }
    fn lighter_colour() -> Color { Self::hex("#b992ff") }
    fn text()           -> Color { Self::hex("#c0caf5") }
    fn darker()         -> Color { Self::hex("#565f89") }
}

struct OxidriveStyle;
impl OxidriveStyle {
    fn background() -> iced::widget::container::Style {
        iced::widget::container::Style {
            background: Some(iced::Background::Color(OxidriveColour::background())),
            text_color: Some(OxidriveColour::text()),
            border: Border::default(),
            shadow: Shadow::default()
        }
    }

    fn search_bar(state: iced::widget::text_input::Status) -> iced::widget::text_input::Style {
        iced::widget::text_input::Style {
            background: iced::Background::Color(match state {
                text_input::Status::Active => OxidriveColour::foreground(),
                text_input::Status::Disabled => OxidriveColour::foreground(),
                _ => OxidriveColour::accent()
            }),
            border: Border::default(),
            icon: OxidriveColour::text(),
            placeholder: OxidriveColour::darker(),
            value: OxidriveColour::text(),
            selection: OxidriveColour::colour()
        }
    }

    fn list_button(state: iced::widget::button::Status) -> iced::widget::button::Style {
        iced::widget::button::Style {
            background: Some(iced::Background::Color(
                match state {
                    iced::widget::button::Status::Active => OxidriveColour::colour(),
                    iced::widget::button::Status::Disabled => OxidriveColour::foreground(),
                    _ => OxidriveColour::lighter_colour()
                }
            )),
            text_color: OxidriveColour::text(),
            border: Border::default(),
            shadow: Shadow::default()
        }
    }

    fn scrollable(_state: iced::widget::scrollable::Status) -> iced::widget::scrollable::Style {
        iced::widget::scrollable::Style {
            container: iced::widget::container::Style {
                background: Some(iced::Background::Color(OxidriveColour::foreground())),
                text_color: Some(OxidriveColour::text()),
                border: Border::default(),
                shadow: Shadow::default()
            },
            vertical_rail: iced::widget::scrollable::Rail {
                background: Some(iced::Background::Color(OxidriveColour::foreground())),
                border: Border::default().rounded(10),
                scroller: Scroller {
                    border: Border::default().rounded(10),
                    color: OxidriveColour::colour()
                }
            },
            horizontal_rail: iced::widget::scrollable::Rail {
                background: Some(iced::Background::Color(OxidriveColour::foreground())),
                border: Border::default().rounded(10),
                scroller: Scroller {
                    border: Border::default().rounded(10),
                    color: OxidriveColour::colour()
                }
            },
            gap: None
        }
    }
}

pub struct OxidriveWidget;
impl OxidriveWidget {
    pub fn window<'a>(widgets: Element<'a, Message>) -> Element<'a, Message> {
        Container::new(widgets).style(|_| OxidriveStyle::background()).padding(20)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn dir_entry_widget<'a>(path: &'a Path) -> Option<Button<'a, Message>> {
        if let Some(filename) = path.file_name() {
            Some(button(
                Row::new()
                    .spacing(20)
                    .width(Length::Fill)
                    .align_y(Vertical::Center)
                    .push(
                        text(filename.to_string_lossy().to_string()).width(Length::FillPortion(5))
                    ).push(
                        match path.is_file() {
                            true => text(
                                format_raw_size(path.metadata().unwrap().st_size() as usize)
                            ),
                            false => text("DIRECTORY")
                        }.width(Length::FillPortion(2))
                    )
            ).style(|_,state| OxidriveStyle::list_button(state))
            )
        } else {
            None
        }
    }

    pub fn search_bar<'a>(default: &'a str, current: &'a str) -> TextInput<'a, Message> {
        text_input(default, current)
            .width(Length::Fill)
            .size(25)
            .style(|_,state| OxidriveStyle::search_bar(state))
    }

    pub fn scrollable<'a>(column: Column<'a, Message>) -> Scrollable<'a, Message> {
        Scrollable::new(column).style(|_,state| OxidriveStyle::scrollable(state)).spacing(20)
    }
}
