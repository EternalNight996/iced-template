use iced::{
  widget::{column, container, horizontal_space, row, text_input, vertical_space},
  window::{self},
  Command, Length, Size,
};

use super::{
  components::helpers::{button2, text2},
  main_app::AppWindow,
};
use crate::{
  config::{
    logger::{log, Tag},
    theme,
  },
  data::DB_SQLITE,
  db::{apis::user::verify_password, r#type::user::UserCfg, DbPool},
  ui::components::Element,
};

#[derive(Clone, Debug)]
pub enum Message {
  SetUserName(String),
  SetUserPasswd(String),
  Submit,
  Result(bool),
}

/// MyApp graphical application
#[derive(Debug)]
pub struct App {
  pub view: super::main_app::View,
  pub flag: UserCfg,
  pub id: window::Id,
  pub passwd: String,
  pub name: String,
}
impl Default for App {
  fn default() -> Self {
    Self {
      view: super::main_app::View::Login,
      id: window::Id::MAIN,
      flag: UserCfg {
        height: 300.0,
        width: 500.0,
        decorations: true,
        ..Default::default()
      },
      passwd: String::new(),
      name: String::new(),
    }
  }
}
impl AppWindow for App {
  type Event = Message;
  type Flag = UserCfg;
  type Setting = UserCfg;

  fn new(flag: Self::Flag) -> Self {
    let mut slf = Self::default();
    slf.flag = flag;
    slf
  }

  fn settings(cfg: Self::Setting) -> window::Settings {
    let size = Size::new(cfg.width, cfg.height);
    window::Settings {
      icon: super::main_app::application_icon(),
      size,
      min_size: Some(size),
      position: window::Position::Centered,
      resizable: cfg.resizable,
      decorations: cfg.decorations,
      transparent: cfg.transparent,
      exit_on_close_request: true,
      ..Default::default()
    }
  }

  fn title(&self) -> String {
    format!("登录窗口")
  }

  fn create(&mut self) -> Command<Self::Event> {
    if self.exist() {
      Command::batch([self.show(), self.focus()])
    } else {
      let (id, spawn_window) = window::spawn(Self::settings(self.flag.clone()));
      self.id = id;
      spawn_window
    }
  }

  fn update(&mut self, event: Self::Event) -> Command<Self::Event> {
    match event {
      Message::SetUserName(word) => self.name = word,
      Message::SetUserPasswd(word) => self.passwd = word,
      Message::Submit => {
        let uname = self.name.clone();
        let passwd = self.passwd.clone();
        return Command::perform(
          async move {
            let pool = DB_SQLITE.read().await.pool();
            iced::futures::executor::block_on(login(&pool, &uname, &passwd))
          },
          Message::Result,
        );
      }
      Message::Result(_) => (),
    }
    Command::none()
  }

  fn view(&self) -> Element<'_, Self::Event> {
    // 使用 Checkbox 创建一个 Column，注意这里的消息类型是 `Message`
    container(
      column![
        row![
          "用户名：",
          text_input("请输入用户名", &self.name).on_input(Message::SetUserName)
        ],
        row![
          "密码    : ",
          text_input("请输入密码", &self.passwd)
            .secure(true)
            .on_submit(Message::Submit)
            .on_input(Message::SetUserPasswd)
        ],
        vertical_space(),
        row![
          horizontal_space(),
          button2(text2("提交").size(25)).width(150).on_press(Message::Submit),
          horizontal_space(),
        ]
      ]
      .spacing(2),
    )
    .padding(10)
    .center_x()
    .align_x(iced::alignment::Horizontal::Center)
    .width(Length::Fill)
    .height(Length::Fill)
    .style(theme::Container::Default)
    .into()
  }

  fn subscription(&self) -> iced::Subscription<Self::Event> {
    iced::Subscription::batch([])
  }

  fn close(&mut self) -> Command<Self::Event> {
    window::close(self.id)
  }

  fn show(&self) -> Command<Self::Event> {
    window::change_mode(self.id, window::Mode::Windowed)
  }

  fn exist(&self) -> bool {
    self.id != window::Id::MAIN
  }

  fn focus(&self) -> Command<Self::Event> {
    window::gain_focus(self.id)
  }
}

/// 访问数据校验数据
pub async fn login(pool: &DbPool, name: &str, passwd: &str) -> bool {
  if !name.is_empty() || !passwd.is_empty() {
    match verify_password(&pool, &name, passwd).await {
      Ok(status) => {
        if status {
          log::info(format!("用户：{} 登录成功！", name), Tag::Login)
        } else {
          log::a_error_box("登录", format!("用户：{} 密码错误", name), Tag::Login).await
        }
        return status;
      }
      Err(_e) => log::a_warn_box("登录", format!("查询账号不存在: {name}"), Tag::Login).await,
    }
  }
  return false;
}
