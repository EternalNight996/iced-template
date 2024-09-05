use iced::Length;

use super::{Button, Container, Text};
use crate::config::theme;

pub fn text<'a>() -> Text<'a> {
  Text::new("")
}
///
pub fn button<'a, Message>() -> Button<'a, Message> {
  Button::new("").style(theme::Button::Hyperlink)
}

pub fn container<'a, Message>() -> Container<'a, Message> {
  Container::new(text()).width(Length::Fill).height(Length::Fill)
}

pub fn not_found<'a, Message>() -> Container<'a, Message> {
  Container::new(Text::new("无法找到ID"))
    .center_x()
    .center_y()
    .align_x(iced::alignment::Horizontal::Center)
    .align_y(iced::alignment::Vertical::Center)
    .width(Length::Fill)
    .height(Length::Fill)
}
