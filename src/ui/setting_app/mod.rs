use iced::{
  widget::{column, horizontal_rule, row, Space},
  window::{self},
  Alignment, Command, Length, Size,
};
mod base;
mod user;
use super::{
  components::{helpers::text_with_logo, layout, Row},
  main_app::AppWindow,
};
use crate::{
  config::{
    theme::{self, Themes},
    Config,
  },
  data::icon,
  db::r#type::user::UserCfg,
  ui::components::{helpers::button2, Element},
};

/// This is basically the configuration panel view.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
  #[default]
  Main,
  User,
}
impl View {
  /// 校验主题
  pub fn get_theme(&self, v: &View) -> theme::Button {
    if self.eq(v) {
      theme::Button::Pressed
    } else {
      theme::Button::Ready
    }
  }
}
/// MyApp graphical application
#[derive(Debug)]
pub struct App {
  pub view: View,
  pub flag: Config,
  pub id: window::Id,
}
impl Default for App {
  fn default() -> Self {
    Self {
      id: window::Id::MAIN,
      view: View::Main,
      flag: Config::default(),
    }
  }
}

#[derive(Debug, Clone)]
pub enum Message {
  UserPressed,
  MainViewPressed,
  SetUserName(String),
  SetUserPasswd(String),
  SetTheme(Themes),
  Save,
  ReLoad,
}

pub fn view(app: &App) -> Element<'_, Message> {
  let cont = layout::content(
    match app.view {
      View::Main => base::view(&app.flag),
      View::User => user::view(&app.flag),
    }
    .push(bts()),
  );
  let side = layout::sidebar(
    column![
      button2(text_with_logo(icon::setting(), "基础"))
        .width(Length::Fill)
        .style(app.view.get_theme(&View::Main))
        .on_press(Message::MainViewPressed),
      button2(text_with_logo(icon::user(), "用户"))
        .width(Length::Fill)
        .style(app.view.get_theme(&View::User))
        .on_press(Message::UserPressed)
    ]
    .align_items(Alignment::Center)
    .width(100),
  );
  Row::new().push(side).push(cont).into()
}

pub fn update(app: &mut App, message: Message) -> Command<Message> {
  let ref mut cfg = app.flag;
  match message {
    Message::UserPressed => app.view = View::User,
    Message::MainViewPressed => app.view = View::Main,
    Message::SetUserName(v) => cfg.user.name = v,
    Message::SetUserPasswd(v) => cfg.user.password = v,
    Message::SetTheme(theme) => cfg.user_cfg.theme = theme,
    Message::Save | Message::ReLoad => (),
  }
  Command::none()
}

/// 提交
pub fn bts() -> Element<'static, Message> {
  column![
    Space::with_height(Length::Fill),
    horizontal_rule(1),
    row![
      Space::with_width(Length::Fill),
      button2(text_with_logo(icon::save(), "保存"))
        .height(50)
        .width(90)
        .on_press(Message::Save),
      button2(text_with_logo(icon::reload(), "加载"))
        .height(50)
        .width(90)
        .on_press(Message::ReLoad),
      Space::with_width(Length::Fill),
    ]
    .spacing(15)
    .align_items(iced::Alignment::Center)
  ]
  .into()
}

impl AppWindow for App {
  type Event = Message;
  type Flag = Config;
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
      position: window::Position::Default,
      resizable: cfg.resizable,
      decorations: cfg.decorations,
      transparent: cfg.transparent,
      exit_on_close_request: true,
      ..Default::default()
    }
  }

  fn title(&self) -> String {
    format!("配置")
  }

  fn create(&mut self) -> Command<Self::Event> {
    if self.exist() {
      Command::batch([self.show(), self.focus()])
    } else {
      let (id, spawn_window) = window::spawn(Self::settings(self.flag.user_cfg.clone()));
      self.id = id;
      spawn_window
    }
  }

  fn update(&mut self, event: Self::Event) -> Command<Self::Event> {
    update(self, event)
  }

  fn view(&self) -> Element<'_, Self::Event> {
    view(self)
  }

  fn subscription(&self) -> iced::Subscription<Self::Event> {
    iced::Subscription::batch([])
  }

  fn close(&mut self) -> Command<Self::Event> {
    window::close(self.id)
  }

  fn exist(&self) -> bool {
    self.id != window::Id::MAIN
  }

  fn show(&self) -> Command<Self::Event> {
    window::change_mode(self.id, window::Mode::Windowed)
  }

  fn focus(&self) -> Command<Self::Event> {
    window::gain_focus(self.id)
  }
}
