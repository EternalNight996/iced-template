//! Helper functions to construct widgets
#![allow(unused)]
use std::borrow::{Borrow, Cow};

use iced::{
  alignment::{Horizontal, Vertical},
  widget::{button, container, row, text},
  Alignment,
  Length,
};

use super::*;
use crate::config::theme;

/// 自定义
pub fn text_with_logo<'a, Message: 'static>(
  logo: impl Into<Element<'a, Message>>,
  content: impl ToString,
) -> Element<'a, Message> {
  Row::new()
    .push(logo)
    .push(
      text(content)
        .horizontal_alignment(Horizontal::Left)
        .vertical_alignment(Vertical::Center)
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .height(35)
    .align_items(Alignment::Start)
    .into()
}

/// 自定义
pub fn button2<'a, Message>(content: impl Into<Element<'a, Message>>) -> Button<'a, Message> {
  centered_button(content).width(40).style(theme::Button::Primary)
}

/// 自定义
/// Creates a new [`Text`] widget with the provided content.
///
/// [`Text`]: core::widget::Text
pub fn text2<'a>(content: impl Into<Cow<'a, str>>) -> Text<'a> {
  Text::new(content)
    .style(theme::Text::Default)
    .vertical_alignment(Vertical::Center)
    .horizontal_alignment(Horizontal::Center)
    .width(Length::Fill)
    .height(Length::Fill)
}

/// TODO
pub fn centered_button<'a, Message>(content: impl Into<Element<'a, Message>>) -> Button<'a, Message> {
  button(content)
}
pub fn smol_button<'a, Message>(content: impl Into<Element<'a, Message>>) -> Button<'a, Message> {
  button(content).height(Length::Shrink)
}

pub fn action<'a, Message>(
  content: impl Into<Element<'a, Message>>,
  message: Option<Message>,
) -> Button<'a, Message> {
  button(content).on_press_maybe(message)
}

pub fn centered_text<'a>(input: impl ToString) -> Text<'a> {
  text(input).horizontal_alignment(Horizontal::Center)
}

pub fn warning<'a>(predicate: impl Fn() -> bool, warning: impl ToString) -> Option<Text<'a>> {
  predicate().then_some(text(warning).style(theme::Text::Warning))
}

pub fn centered_container<'a, Message>(content: impl Into<Element<'a, Message>>) -> Container<'a, Message> {
  container(content)
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
}

pub fn fill_container<'a, Message>(content: impl Into<Element<'a, Message>>) -> Container<'a, Message> {
  container(content).width(Length::Fill).height(Length::Fill)
}

/// MyApp control helper widget
pub fn control<'a, Message: 'a>(
  title: impl Into<Element<'a, Message>>,
  content: impl Into<Element<'a, Message>>,
) -> Container<'a, Message> {
  container(
    Column::new().padding(8).spacing(8).push(title).push(
      container(content)
        .padding(8)
        // .style(theme::Container::Frame)
        .width(Length::Fill),
    ),
  )
}

pub fn control_filled<'a, Message: 'a>(
  title: impl Into<Element<'a, Message>>,
  content: impl Into<Element<'a, Message>>,
) -> Container<'a, Message> {
  fill_container(
    Column::new().spacing(8).push(title).push(
      container(content)
        .padding(8)
        .style(theme::Container::Frame)
        .width(Length::Fill)
        .height(Length::Fill),
    ),
  )
}

pub fn labelled_picklist<'a, Message, T, L, V, F>(
  label: impl ToString,
  options: L,
  selected: Option<V>,
  on_selected: F,
) -> Element<'a, Message>
where
  Message: Clone + 'a,
  T: ToString + Eq + Clone + 'a,
  L: Borrow<[T]> + 'a,
  V: Borrow<T> + 'a,
  F: Fn(T) -> Message + 'a,
{
  row![PickList::new(options, selected, on_selected), text(label)]
    .align_items(Alignment::Center)
    .spacing(8)
    .into()
}
