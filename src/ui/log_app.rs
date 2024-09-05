use iced::{
  widget::{
    container,
    scrollable::{self, Properties},
    text, Column,
  },
  window::{self},
  Command, Length, Size,
};

use super::main_app::AppWindow;
use crate::{
  config::{
    logger::{log, Tag},
    theme,
  },
  db::r#type::user::UserCfg,
  ui::components::Element,
};


#[derive(Clone, Debug)]
pub enum Message {
}

/// MyApp graphical application
#[derive(Debug)]
pub struct App {
  pub flag: UserCfg,
  pub id: window::Id,
  pub state: bool,
}
impl Default for App {
  fn default() -> Self {
    Self {
      id: window::Id::MAIN,
      flag: UserCfg::default(),
      state: false,
    }
  }
}
impl App {
  /// 更新state
  pub fn update_log_window_state(&mut self, status: bool) {
    self.state = status;
    log::info(
      format!("日志窗口 {}", if status { "开启" } else { "关闭" }),
      Tag::Window,
    );
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
      position: window::Position::Default,
      resizable: cfg.resizable,
      decorations: cfg.decorations,
      transparent: cfg.transparent,
      exit_on_close_request: true,
      ..Default::default()
    }
  }

  fn title(&self) -> String {
    format!("日志窗口")
  }

  fn create(&mut self) -> Command<Self::Event> {
    self.update_log_window_state(true);
    if self.exist() {
      Command::batch([self.show(), self.focus()])
    } else {
      let (id, spawn_window) = window::spawn(Self::settings(self.flag.clone()));
      self.id = id;
      spawn_window
    }
  }
  ///
  fn update(&mut self, _event: Self::Event) -> Command<Self::Event> {
    Command::none()
  }
  /// 日志窗口组件
  fn view(&self) -> Element<'_, Self::Event> {
    let data = log::list(0);
    // 使用 Checkbox 创建一个 Column，注意这里的消息类型是 `Message`
    container(
      scrollable::Scrollable::new(
        Column::with_children(data.into_iter().enumerate().map(|(i, (datetime, level, msg))| {
          container(text(format!("{} {datetime} [{level}] {msg}", i + 1,)).style(theme::Text::from(&level))).into()
        }))
        .padding(10)
        .spacing(10), // 添加间距
      )
      .direction(scrollable::Direction::Both {
        vertical: Properties::new().alignment(scrollable::Alignment::End),
        horizontal: Properties::new().alignment(scrollable::Alignment::Start),
      })
      .width(Length::Fill)
      .height(Length::Fill)
      .style(theme::Scrollable::Normal),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(theme::Container::Black)
    .into()
  }

  fn subscription(&self) -> iced::Subscription<Self::Event> {
    iced::Subscription::batch([])
  }

  fn close(&mut self) -> Command<Self::Event> {
    self.update_log_window_state(false);
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
