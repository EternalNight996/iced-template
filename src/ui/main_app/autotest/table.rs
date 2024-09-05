use crate::config::{theme, Config};
use crate::db::r#type::app::ExtendApp;
use crate::ui::components::table::table::{self};
use crate::ui::main_app::AppWindow;
use iced::widget::{checkbox, column, container, horizontal_space, responsive, row, scrollable, text};
use iced::{window, Command, Length, Size};
use serde_json::Value;

use crate::ui::components::{Element, Renderer, Theme};

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
        label: format!("{i}"),
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

#[derive(Debug, Clone)]
pub enum Message {
  SyncHeader(scrollable::AbsoluteOffset),
  Resizing(usize, f32),
  Resized,
  ResizeColumnsEnabled(bool),
  FooterEnabled(bool),
  MinWidthEnabled(bool),
}
pub struct App {
  id: window::Id,
  columns: Vec<Column>,
  rows: Vec<Data>,
  header: scrollable::Id,
  body: scrollable::Id,
  footer: scrollable::Id,
  resize_columns_enabled: bool,
  footer_enabled: bool,
  min_width_enabled: bool,
  title: String,
  pub flag: Config,
}

impl Default for App {
  fn default() -> Self {
    Self {
      id: window::Id::MAIN,
      columns: vec![
        Column::new(ColumnKind::Index),
        Column::new(ColumnKind::Label),
        Column::new(ColumnKind::State),
        Column::new(ColumnKind::Value),
        Column::new(ColumnKind::ResValue),
        Column::new(ColumnKind::Type),
        Column::new(ColumnKind::Priority),
        Column::new(ColumnKind::IsCheck),
        Column::new(ColumnKind::IsRepeat),
        Column::new(ColumnKind::IsWait),
        Column::new(ColumnKind::Timeout),
        Column::new(ColumnKind::Count),
      ],
      rows: (0..150).map(Data::generate).collect(),
      header: scrollable::Id::unique(),
      body: scrollable::Id::unique(),
      footer: scrollable::Id::unique(),
      resize_columns_enabled: true,
      footer_enabled: true,
      min_width_enabled: true,
      title: String::new(),
      flag: Config::default(),
    }
  }
}

impl AppWindow for App {
  type Event = Message;
  type Flag = Config;
  type Setting = Config;

  fn new(flag: Self::Flag) -> Self {
    let mut slf = Self::default();
    slf.flag = flag;
    slf
  }

  fn settings(flag: Self::Setting) -> window::Settings {
    let ref cfg = flag.user_cfg;
    let size = Size::new(cfg.width, cfg.height);
    window::Settings {
      icon: crate::ui::main_app::application_icon(),
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
    self.title.clone()
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
      Message::SyncHeader(offset) => {
        return Command::batch(vec![
          scrollable::scroll_to(self.header.clone(), offset),
          scrollable::scroll_to(self.footer.clone(), offset),
        ])
      }
      Message::Resizing(index, offset) => {
        if let Some(column) = self.columns.get_mut(index) {
          column.resize_offset = Some(offset);
        }
      }
      Message::Resized => self.columns.iter_mut().for_each(|column| {
        if let Some(offset) = column.resize_offset.take() {
          column.width += offset;
        }
      }),
      Message::ResizeColumnsEnabled(enabled) => self.resize_columns_enabled = enabled,
      Message::FooterEnabled(enabled) => self.footer_enabled = enabled,
      Message::MinWidthEnabled(enabled) => self.min_width_enabled = enabled,
    }

    Command::none()
  }

  /// 日志窗口组件
  fn view(&self) -> Element<'_, Self::Event> {
    let table = responsive(|size| {
      let mut table = table::table(
        self.header.clone(),
        self.body.clone(),
        &self.columns,
        &self.rows,
        Message::SyncHeader,
      );
      if self.resize_columns_enabled {
        table = table.on_column_resize(Message::Resizing, Message::Resized);
      }
      if self.footer_enabled {
        table = table.footer(self.footer.clone());
      }
      if self.min_width_enabled {
        table = table.min_width(size.width);
      }
      table.into()
    });

    let content = column![
      row![
        checkbox("自定义键宽度", self.resize_columns_enabled,).on_toggle(Message::ResizeColumnsEnabled),
        checkbox("显示底部", self.footer_enabled,).on_toggle(Message::FooterEnabled),
        checkbox("自适应宽度", self.min_width_enabled,).on_toggle(Message::MinWidthEnabled),
      ],
      table
    ]
    .spacing(6);

    container(container(content).width(Length::Fill).height(Length::Fill))
      .padding(20)
      .width(Length::Fill)
      .height(Length::Fill)
      .center_y()
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

#[derive(Debug)]
pub struct Column {
  kind: ColumnKind,
  width: f32,
  resize_offset: Option<f32>,
}

impl Column {
  pub fn new(kind: ColumnKind) -> Self {
    let width = match kind {
      ColumnKind::Index => 50.0,
      ColumnKind::Label => 140.0,
      ColumnKind::State => 140.0,
      ColumnKind::Type => 60.0,
      ColumnKind::Value => 150.0,
      ColumnKind::ResValue => 150.0,
      ColumnKind::Priority => 70.0,
      ColumnKind::IsCheck => 50.0,
      ColumnKind::IsRepeat => 50.0,
      ColumnKind::IsWait => 50.0,
      ColumnKind::Timeout => 60.0,
      ColumnKind::Count => 80.0,
    };

    Self {
      kind,
      width,
      resize_offset: None,
    }
  }
}
#[derive(Debug)]
pub enum ColumnKind {
  Index,
  Label,
  State,
  Value,
  ResValue,
  Type,
  Priority,
  IsCheck,
  IsRepeat,
  IsWait,
  Timeout,
  Count,
}

impl<'a> table::Column<'a, Message, Theme, Renderer> for Column {
  type Row = Data;
  fn header(&'a self, _col_index: usize) -> Element<'a, Message> {
    let content = match self.kind {
      ColumnKind::Index => "序号",
      ColumnKind::Label => "标签",
      ColumnKind::State => "状态",
      ColumnKind::Value => "标准值",
      ColumnKind::ResValue => "结果值",
      ColumnKind::Type => "类型",
      ColumnKind::Priority => "优先级",
      ColumnKind::IsCheck => "是否检查",
      ColumnKind::IsRepeat => "是否重复",
      ColumnKind::IsWait => "是否等待",
      ColumnKind::Timeout => "延迟",
      ColumnKind::Count => "次数",
    };

    container(text(content))
      .height(24)
      .width(Length::Fill)
      .center_x()
      .center_y()
      .into()
  }
  fn cell(&'a self, _col_index: usize, row_index: usize, row: &'a Self::Row) -> Element<'a, Message> {
    let ref app = row.extend_app;
    let content: Element<'_, _> = match self.kind {
      ColumnKind::Index => text(row_index + 1).into(),
      ColumnKind::Label => text(&format!("{}", app.label)).into(),
      ColumnKind::State => match row.state {
        DataState::Ready => container("Ready").style(theme::Container::Default),
        DataState::Success => container("Pass").style(theme::Container::Success),
        DataState::Fail => container("Fail").style(theme::Container::Error),
      }
      .center_x()
      .width(Length::Fill)
      .into(),
      ColumnKind::Value => text(&row.value).into(),
      ColumnKind::ResValue => text(&row.res_value).into(),
      ColumnKind::Type => text(&app.r#type).into(),
      ColumnKind::Priority => text(app.priority.to_string()).into(),
      ColumnKind::IsCheck => text(if app.is_check { "是" } else { "否" }).into(),
      ColumnKind::IsRepeat => text(if app.is_repeat { "是" } else { "否" }).into(),
      ColumnKind::IsWait => text(if app.is_wait { "是" } else { "否" }).into(),
      ColumnKind::Timeout => text(&app.timeout.to_string()).into(),
      ColumnKind::Count => text(&app.count.to_string()).into(),
    };

    container(content)
      .width(Length::Fill)
      .height(32)
      .center_x()
      .center_y()
      .into()
  }

  fn footer(&'a self, _col_index: usize, rows: &'a [Self::Row]) -> Option<Element<'a, Message>> {
    let content = if matches!(self.kind, ColumnKind::State) {
      let total = rows.len();
      Element::from(text(format!("Total: {total}")))
    } else {
      horizontal_space().into()
    };

    Some(container(content).height(24).center_y().into())
  }

  fn width(&self) -> f32 {
    self.width
  }

  fn resize_offset(&self) -> Option<f32> {
    self.resize_offset
  }
}
