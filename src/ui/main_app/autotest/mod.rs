mod table;
pub mod theme;
use super::AppWindow;
use crate::{
  config::Config,
  db::r#type::app::ExtendApp,
  ui::components::{
    helpers::{button2, text2},
    Container, Element, Text,
  },
};
use iced::widget::{column, horizontal_space, row};
use iced::{
  widget::{container, text},
  window, Command, Length, Size,
};
use serde_json::Value;

#[derive(Clone, Debug)]
pub enum Message {
  TableWindow(table::Message),
  Submit,
}

#[derive(Debug, Clone)]
pub struct Data {
  pub state: DataState,
  pub value: Value,
  pub res_value: Value,
  pub extend_app: ExtendApp,
}
impl Data {
  pub fn generate(i: i32) -> Self {
    Self {
      state: match i % 2 {
        0 if i == 0 => DataState::Ready,
        1 => DataState::Fail,
        _ => DataState::Success,
      },
      value: Value::Number(i.into()),
      res_value: Value::Null,
      extend_app: ExtendApp {
        tag: format!("{i}"),
        ..Default::default()
      },
    }
  }
}
#[derive(Debug, Clone)]
pub enum DataState {
  Ready,
  Success,
  Fail,
}
impl DataState {
  pub fn to_text<'a>(&'a self, size: impl Into<iced::Pixels>) -> Text<'a> {
    match self {
      DataState::Ready => text("Ready").size(size).style(crate::config::theme::Text::Default),
      DataState::Success => text("Pass").size(size).style(crate::config::theme::Text::Success),
      DataState::Fail => text("Fail").size(size).style(crate::config::theme::Text::Error),
    }
    .into()
  }
  pub fn to_container<'a, Event>(&'a self, size: impl Into<iced::Pixels>) -> Container<'a, Event> {
    match self {
      DataState::Ready => container(text("Ready").size(size)).style(crate::config::theme::Container::Default),
      DataState::Success => container(text("Pass").size(size)).style(crate::config::theme::Container::Success),
      DataState::Fail => container(text("Fail").size(size)).style(crate::config::theme::Container::Error),
    }
    .center_x()
    .center_y()
    .width(Length::Fill)
    .into()
  }
}
/// MyApp graphical application
pub struct App {
  pub flag: Config,
  pub id: window::Id,
  pub state: DataState,
  pub table_window: table::App,
}
impl Default for App {
  fn default() -> Self {
    Self {
      id: window::Id::MAIN,
      flag: Config::default(),
      state: DataState::Success,
      table_window: table::App::new(Config::default()),
    }
  }
}

impl AppWindow for App {
  type Event = Message;
  type Flag = Config;
  type Setting = Config;

  fn new(flag: Self::Flag) -> Self {
    let mut slf = Self::default();
    slf.table_window.flag = flag.clone();
    slf.flag = flag;
    slf
  }

  fn settings(flag: Self::Setting) -> window::Settings {
    let ref cfg = flag.user_cfg;
    let size = Size::new(cfg.width, cfg.height);
    window::Settings {
      icon: super::application_icon(),
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
    format!("主窗口")
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
      Message::TableWindow(event) => {
        let command = self.table_window.update(event);
        command.map(Message::TableWindow)
      }
      _ => Command::none(),
    }
  }

  /// 日志窗口组件
  fn view(&self) -> Element<'_, Self::Event> {
    let submit = row![
      horizontal_space(),
      button2(text2("提交").size(25)).width(150).on_press(Message::Submit),
      horizontal_space(),
    ]
    .height(60);

    column![self.table_window.view().map(Message::TableWindow), submit].into()
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

  fn focus(&mut self) -> Command<Self::Event> {
    window::gain_focus(self.id)
  }
}
