use iced::{
  multi_window::{self, Application},
  widget::{column, horizontal_space, row, text},
  window, Alignment, Command, Length, Size, Subscription,
};
use std::collections::HashMap;
use tokio::time::Instant;
pub mod autotest;
use super::{
  components::{
    helpers::{button2, text_with_logo},
    layout,
  },
  log_app, login_app, setting_app,
};
use crate::{
  config::{
    logger::{log, Tag},
    theme, Config,
  },
  data::{
    font,
    icon::{self, title_logo},
    DB_SQLITE, MAIN_LOGO,
  },
  db::{
    apis::{load_all_data, save_all_data},
    r#type::user::UserCfg,
  },
  event,
  res::PanicAny,
  ui::components::{empty, Element},
};

#[derive(Clone, Debug)]
pub enum Message {
  Event(event::Event),
  SaveConfig,
  LoadConfig,
  LoadConfigResult(Result<Config, String>),
  SaveConfigResult(Result<Config, String>),
  FontLoaded(Result<(), iced::font::Error>),
  LogEvent(log_app::Message),
  LoginEvent(login_app::Message),
  AutotestEvent(autotest::Message),
  SettingEvent(setting_app::Message),
  Login(View),
  LogPressed,
  MainPressed,
  Ignore,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum View {
  Main,
  Log,
  Setting,
  Login,
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
pub trait AppWindow: 'static + Sized {
  type Flag;
  type Setting;
  type Event;
  fn new(flag: Self::Flag) -> Self;
  fn settings(cfg: Self::Setting) -> window::Settings;
  fn title(&self) -> String;
  fn create(&mut self) -> Command<Self::Event>;
  fn close(&mut self) -> Command<Self::Event>;
  fn show(&self) -> Command<Self::Event>;
  fn focus(&mut self) -> Command<Self::Event>;
  fn exist(&self) -> bool;
  fn update(&mut self, event: Self::Event) -> Command<Self::Event>;
  fn view(&self) -> Element<'_, Self::Event>;
  fn subscription(&self) -> Subscription<Self::Event>;
}
/// App graphical application
pub struct App {
  pub id: window::Id,
  pub focused_id: window::Id,
  pub view: View,
  pub flag: Config,
  pub log_app: log_app::App,
  pub login_app: login_app::App,
  pub setting_app: setting_app::App,
  pub autotest: autotest::App,
  pub window_flags: HashMap<window::Id, View>,
  pub run_time: Instant,
}

impl App {
  /// Launch the application
  pub fn launch() -> crate::Result<()> {
    // load configuration
    let mut config = Config::default();
    config.init_base_conf();
    let (sub, _guards) = config.log.get_subscriber(config.log.level);
    config.log.init(sub).panic("Init log");
    Self::run(Self::setting_global(config))?;
    Ok(())
  }
}
impl App {
  pub fn setting_global(config: Config) -> iced::Settings<Config> {
    let ref cfg = config.user_cfg;
    let size = Size::new(cfg.width, cfg.height);
    iced::Settings {
      default_text_size: cfg.default_text_size.into(),
      antialiasing: false,
      default_font: iced::Font::with_name("阿里妈妈东方大楷"),
      window: window::Settings {
        icon: super::main_app::application_icon(),
        size,
        min_size: Some(size),
        position: window::Position::Centered,
        resizable: cfg.resizable,
        decorations: cfg.decorations,
        transparent: cfg.transparent,
        ..Default::default()
      },
      flags: config,
      ..Default::default()
    }
  }

  /// 更新
  pub fn update_cfg(&mut self, flag: Config) {
    self.setting_app.flag = flag.clone();
    self.log_app.flag = flag.user_cfg.clone();
    self.flag = flag;
  }

  /// 移除并close
  pub fn close(&mut self, id: window::Id) -> Command<Message> {
    if let Some(_) = self.window_flags.remove(&id) {
      log::debug(format!("尝试关闭窗口 ID[{id:?}]"), Tag::Window);
      match id {
        x if x == self.id => self.close(self.id),
        x if x == self.setting_app.id => {
          let e = self.setting_app.close().map(Message::SettingEvent);
          self.setting_app.id = window::Id::MAIN;
          e
        }
        x if x == self.log_app.id => {
          let e = self.log_app.close().map(Message::LogEvent);
          self.log_app.id = window::Id::MAIN;
          e
        }
        x if x == self.login_app.id => {
          let e = self.login_app.close().map(Message::LoginEvent);
          self.login_app.id = window::Id::MAIN;
          e
        }
        _ => window::close(id),
      }
    } else {
      log::warn(format!("未找到 ID[{id:?}]"), Tag::Window);
      Command::none()
    }
  }

  /// 移除所有ID
  pub fn close_all(&mut self) -> Command<Message> {
    Command::batch(self.window_flags.drain().map(|(id, _winflag)| window::close(id)))
  }

  /// 创建
  pub fn create(&mut self, view: View) -> Command<Message> {
    log::debug(format!("尝试窗口窗口 View[{view:?}]"), Tag::Window);
    match view {
      View::Main => {
        let e = self.create(view);
        self.window_flags.insert(self.id, view);
        e
      }
      View::Log => {
        let e = self.log_app.create().map(Message::LogEvent);
        self.window_flags.insert(self.log_app.id, view);
        e
      }
      View::Setting => {
        let e = self.setting_app.create().map(Message::SettingEvent);
        self.window_flags.insert(self.setting_app.id, view);

        e
      }
      View::Login => {
        let e = self.login_app.create().map(Message::LoginEvent);
        self.window_flags.insert(self.login_app.id, view);
        e
      }
    }
  }
}
/// TODO: allow the user to customize their application icon
pub fn application_icon() -> Option<iced::window::Icon> {
  iced::window::icon::from_file_data(MAIN_LOGO, None).ok()
}

impl multi_window::Application for App {
  type Executor = iced::executor::Default;
  type Flags = Config;
  type Message = Message;
  type Theme = theme::Theme;

  fn new(flags: Self::Flags) -> (Self, Command<Message>) {
    let app = App {
      id: window::Id::MAIN,
      focused_id: window::Id::MAIN,
      view: View::Main,
      log_app: log_app::App::new(flags.user_cfg.clone()),
      login_app: login_app::App::new(UserCfg {
        height: 300.0,
        width: 500.0,
        decorations: true,
        ..Default::default()
      }),
      setting_app: setting_app::App::new(flags.clone()),
      autotest: autotest::App::new(flags.clone()),
      window_flags: HashMap::from_iter([(window::Id::MAIN, View::Main)]),
      run_time: Instant::now(),
      flag: flags,
    };
    (app, Command::batch([font::load().map(Message::FontLoaded)]))
  }

  fn title(&self, id: window::Id) -> String {
    match id {
      x if x == self.id => {
        format!(
          "{} v{}",
          self.flag.cargo.package.description, self.flag.cargo.package.version
        )
      }
      x if x == self.log_app.id => self.log_app.title(),
      x if x == self.setting_app.id => self.setting_app.title(),
      x if x == self.login_app.id => self.login_app.title(),
      _ => "Unknown Window Id".to_string(),
    }
  }

  fn theme(&self, _id: window::Id) -> Self::Theme {
    theme::Theme(self.setting_app.flag.user_cfg.theme.palette()).clone()
  }

  fn view(&self, id: window::Id) -> Element<'_, Message> {
    match id {
      x if x == self.id => self.main_view(),
      x if x == self.login_app.id => self.login_app.view().map(Message::LoginEvent),
      x if x == self.log_app.id => self.log_app.view().map(Message::LogEvent),
      x if x == self.setting_app.id => self.setting_app.view().map(Message::SettingEvent),
      _ => empty::not_found().into(),
    }
  }

  fn update(&mut self, message: Message) -> Command<Message> {
    match message {
      Message::AutotestEvent(e) => return self.autotest.update(e).map(Message::AutotestEvent),
      Message::MainPressed => self.view = View::Main,
      Message::LogPressed => return self.create(View::Log),
      Message::Login(view) => {
        self.login_app.view = view;
        return self.create(View::Login);
      }
      Message::LoginEvent(e) => match e {
        login_app::Message::Result(status) => {
          if status {
            return Command::batch([self.create(self.login_app.view), self.close(self.login_app.id)]);
          }
        }
        _ => return self.login_app.update(e).map(Message::LoginEvent),
      },
      Message::FontLoaded(result) => log::debug(format!("Successful to load font {:?}", result), Tag::LoadSetting),
      Message::SaveConfig => {
        let flag = self.setting_app.flag.clone();
        return Command::perform(
          async move {
            let pool = DB_SQLITE.read().await.pool();
            save_all_data(&pool, flag).await
          },
          Message::SaveConfigResult,
        );
      }
      Message::LoadConfig => {
        return Command::perform(
          async move {
            let pool = DB_SQLITE.read().await.pool();
            // sqlite3
            load_all_data(&pool, 0).await
          },
          Message::LoadConfigResult,
        );
      }
      Message::LoadConfigResult(res) => match res {
        Ok(cfg) => {
          self.update_cfg(cfg.clone());
          a_task!(async move {
            log::info(format!("配置明细： {cfg:#?}"), Tag::DatabaseOffline);
            log::a_info_box("配置", "成功加载配置", Tag::DatabaseOffline).await
          });
        }
        Err(e) => {
          a_task!(async move {
            log::a_error_box("配置", format!("加载配置失败: {e}"), Tag::DatabaseOffline).await
          })
        }
      },
      Message::SaveConfigResult(result) => match result {
        Ok(conf) => {
          self.update_cfg(conf);
          let task2 = Command::perform(
            async move { log::a_info_box("保存配置", "成功保存配置", Tag::DatabaseOffline).await },
            |_| Message::Ignore,
          );
          let task = self.close(self.setting_app.id);
          return Command::batch([task, task2]);
        }
        Err(e) => {
          a_task!(async move {
            log::a_error_box("配置", format!("保存配置失败: {e}"), Tag::DatabaseOffline).await
          })
        }
      },
      Message::Event(event) => match event {
        event::Event::ChangeBit => {
          let ref id = self.focused_id;
          if id.eq(&self.login_app.id) {
            return self.login_app.focus().map(Message::LoginEvent);
          }
        }
        event::Event::Clear => {}
        event::Event::Closed(id) => match id {
          x if x == self.id => return self.close_all(),
          _ => return self.close(id),
        },
        event::Event::CloseRequested => {}
        event::Event::Delete => {}
        event::Event::Save => return self.update(Message::SaveConfig),
        event::Event::Load => return self.update(Message::LoadConfig),
        event::Event::Focused(id) => {
          self.focused_id = id;
          if self.focused_id.eq(&self.login_app.id) {
            return self.login_app.focus().map(Message::LoginEvent);
          }
        }
      },
      Message::LogEvent(e) => return self.log_app.update(e).map(Message::LogEvent),
      Message::SettingEvent(e) => {
        return match e {
          setting_app::Message::Save => self.update(Message::SaveConfig),
          setting_app::Message::ReLoad => self.update(Message::LoadConfig),
          _ => self.setting_app.update(e).map(Message::SettingEvent),
        };
      }
      Message::Ignore => (),
    }
    Command::none()
  }

  fn subscription(&self) -> Subscription<Message> {
    iced::Subscription::batch([
      event::events().map(Message::Event),
      self.log_app.subscription().map(Message::LogEvent),
      self.setting_app.subscription().map(Message::SettingEvent),
    ])
  }
}
impl App {
  fn main_view(&self) -> Element<'_, Message> {
    let header_tools = row![
      text(format!("运行时间: {}/S", self.run_time.elapsed().as_secs())),
      horizontal_space(),
      text("当前工站: IFT")
    ]
    .align_items(Alignment::Center)
    .spacing(10)
    .padding(10);
    let headers = layout::header(title_logo(), self.autotest.state.to_text(40), header_tools);
    let demo_view = layout::content(match self.view {
      View::Setting => self.setting_app.view().map(Message::SettingEvent),
      View::Log => self.log_app.view().map(Message::LogEvent),
      View::Main => self.autotest.view().map(Message::AutotestEvent),
      _ => empty::not_found().into(),
    });
    let side_tools = layout::sidebar(
      column![
        button2(text_with_logo(icon::home(), "主页"))
          .width(Length::Fill)
          .style(self.view.get_theme(&View::Main))
          .on_press(Message::MainPressed),
        button2(text_with_logo(icon::setting(), "配置"))
          .width(Length::Fill)
          .style(self.view.get_theme(&View::Setting))
          .on_press(Message::Login(View::Setting)),
        button2(text_with_logo(icon::log(), "日志"))
          .width(Length::Fill)
          .style(self.view.get_theme(&View::Log))
          .on_press(Message::LogPressed),
      ]
      .align_items(Alignment::Center)
      .width(100),
    );
    layout::layout(headers, side_tools, demo_view).into()
  }
}
