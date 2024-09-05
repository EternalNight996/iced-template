use crate::{config::theme, data::icon};
use iced::Padding;
use iced::{
  widget::{column, container, horizontal_space, row},
  Alignment, Length,
};

use super::{Column, Element, Row};

pub fn header<'a, Message: 'static, Logo, Title, Tools>(
  logo: Logo,
  title: Title,
  tools: Tools,
) -> Element<'a, Message>
where
  Logo: Into<Element<'a, Message>>,
  Title: Into<Element<'a, Message>>,
  Tools: Into<Element<'a, Message>>,
{
  container(
    Column::new()
      .push(Row::new().push(logo).push(horizontal_space()).push(title).padding(10))
      .push(tools),
  )
  .align_x(iced::alignment::Horizontal::Left)
  .style(theme::Container::Default)
  .into()
}

pub fn sidebar<'a, Message: 'static>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
  container(content)
    .style(theme::Container::Frame)
    .height(Length::Fill)
    .into()
}

pub fn content<'a, Message: 'static>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
  container(content)
    .style(theme::Container::Frame)
    .padding(10)
    .height(Length::Fill)
    .into()
}

pub fn layout<'a, Message: 'static, Head, Side, Content>(
  head: Head,
  side: Side,
  cont: Content,
) -> Element<'a, Message>
where
  Head: Into<Element<'a, Message>>,
  Side: Into<Element<'a, Message>>,
  Content: Into<Element<'a, Message>>,
{
  Column::new().push(head).push(Row::new().push(side).push(cont)).into()
}
