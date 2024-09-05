mod table;
pub mod theme;
use super::AppWindow;
use crate::{config::Config, ui::components::Element};
use iced::{window, Command, Size};

#[derive(Clone, Debug)]
pub enum Message {
  TableWindow(table::Message),
}

/// MyApp graphical application
pub struct App {
  pub flag: Config,
  pub id: window::Id,
  pub table_window: table::App,
}
impl Default for App {
  fn default() -> Self {
    Self {
      id: window::Id::MAIN,
      flag: Config::default(),
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
    }
  }

  /// 日志窗口组件
  fn view(&self) -> Element<'_, Self::Event> {
    self.table_window.view().map(Message::TableWindow)
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
