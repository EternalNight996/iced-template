use crate::{
  config::{
    theme::{self, Themes},
    Config,
  },
  data::icon,
  db::r#type::user::UserCfg,
  ui::components::{
    helpers::{control, text_with_logo},
    Column, Element, Row,
  },
};
use iced::widget::{column, container, pick_list, row};

pub fn view(flag: &Config) -> Column<'_, super::Message> {
  column![apps(&flag)].spacing(8)
}

/// 应用配置
pub fn apps(flag: &Config) -> Row<'_, super::Message> {
  row![themes(&flag.user_cfg)].spacing(8)
}

/// 主题
pub fn themes(cfg: &UserCfg) -> Element<'_, super::Message> {
  let settings = row![pick_list(
    Themes::ALL.as_slice(),
    Some(cfg.theme),
    super::Message::SetTheme
  )]
  .spacing(8)
  .align_items(iced::Alignment::Center);
  control(container(text_with_logo(icon::theme(), "主题")), settings)
    .style(theme::Container::Frame)
    .into()
}
