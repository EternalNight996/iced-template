use crate::{
  config::Config,
  data::icon,
  db::r#type::user::{User, UserInfo},
  ui::components::{
    helpers::{control, text_with_logo},
    Column, Element,
  },
};
use iced::widget::{column, container, row, scrollable, text, text_input};

pub fn view(flag: &Config) -> Column<'_, super::Message> {
  column![user_base(&flag.user), user_info(&flag.user_info)].spacing(8)
}

/// 用户名密码
pub fn user_base(cfg: &User) -> Element<'_, super::Message> {
  control(
    container(text_with_logo(icon::edit_square(), "用户")),
    scrollable(column![
      row!["用户ID：", text(format!("{:?}", cfg.id))],
      row![
        "用户名：",
        text_input("请输入用户名", &cfg.name).on_input(super::Message::SetUserName)
      ],
      row![
        "密码   : ",
        text_input("请输入密码", &cfg.password)
          .secure(true)
          .on_input(super::Message::SetUserPasswd)
      ],
      row!["邮箱   ：", text(format!("{:?}", cfg.email))],
      row!["手机前缀：", text(format!("{:?}", cfg.prefix_mobile))],
      row!["手机号：", text(format!("{:?}", cfg.mobile))],
      row!["上次更新：", text(format!("{:?}", cfg.updated_at))],
      row!["创建更新：", text(format!("{:?}", cfg.created_at))],
    ]),
  )
  .into()
}

/// 用户信息
pub fn user_info(cfg: &UserInfo) -> Element<'_, super::Message> {
  control(
    container(text_with_logo(icon::edit_square(), "用户信息")),
    scrollable(column![
      row!["昵称：", text(format!("{:?}", cfg.nickname))],
      row!["头像：", text(format!("{:?}", cfg.avatar_url))],
      row!["描述：", text(format!("{:?}", cfg.description))],
      row!["标识：", text(format!("{:?}", cfg.identity))],
      row!["状态：", text(format!("{:?}", cfg.status))],
      row!["上次更新：", text(format!("{:?}", cfg.updated_at))],
    ]),
  )
  .into()
}
