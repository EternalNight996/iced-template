use std::{fmt::Display, path::PathBuf};

use e_log::{
  self,
  __private::Subscriber,
  appender::{self, non_blocking::WorkerGuard},
  init_subscriber,
  subscriber::{self, fmt::time::ChronoUtc, layer::SubscriberExt as _, Registry},
  Level, LogTarget,
};
use e_utils::{
  parse::AutoPath as _,
  time::{DateTime, Utc},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::db::r#type::logger::LogQ;

#[derive(Deserialize, Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LogCfg {
  pub id: i32,
  pub level: Level,
  pub folder: String,
  pub fname: String,
  pub format: String,
  pub output_list: Vec<LogTarget>,
  pub tracing: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
impl From<LogQ> for LogCfg {
  fn from(value: LogQ) -> Self {
    let level = match value.level {
      1 => Level::Error,
      2 => Level::Warn,
      3 => Level::Info,
      4 => Level::Debug,
      5 => Level::Trace,
      _ => Level::Off,
    };
    let output_list: Vec<String> = serde_json::from_value(value.output_list).unwrap_or(vec![]);
    let output_list: Vec<LogTarget> = output_list.iter().filter_map(|x| LogTarget::from_str(x).ok()).collect();
    Self {
      id: value.id,
      level,
      folder: value.folder,
      fname: value.fname,
      format: value.format.unwrap_or("".to_string()),
      output_list,
      tracing: value.tracing,
      created_at: value.created_at,
      updated_at: value.updated_at,
    }
  }
}
impl Into<LogQ> for LogCfg {
  fn into(self) -> LogQ {
    LogQ {
      id: self.id,
      level: self.level as i32,
      folder: self.folder,
      fname: self.fname,
      format: Some(self.format),
      output_list: serde_json::to_value(&self.output_list).unwrap_or_default(),
      tracing: self.tracing,
      created_at: Default::default(),
      updated_at: Default::default(),
    }
  }
}
impl LogCfg {
  /// 解析完整路径
  pub fn get_full_path(&self, flag: &str) -> PathBuf {
    PathBuf::new()
      .join(&self.folder)
      .join(format!("{}{}", flag, self.fname))
  }
}

impl LogCfg {
  /// 初始化
  pub fn init(&self, sub: impl Subscriber + Send + Sync) -> e_utils::Result<()> {
    self.folder.auto_create_dir()?;
    init_subscriber(sub, false);
    e_log::panic::reattach_windows_terminal();
    e_log::panic::set_panic_hook(self.get_full_path("bug."))?;

    Ok(())
  }

  /// Compose multiple layers into a `tracing`'s subscriber.
  pub fn get_subscriber(&self, level: Level) -> (impl Subscriber + Send + Sync, Vec<WorkerGuard>) {
    let roll = appender::rolling::never(&self.folder, &self.fname, e_log::FileShare::Read);
    let (f, guard) = appender::non_blocking(roll);
    let (f2, guard2) = appender::non_blocking(std::io::stdout());
    let timer = ChronoUtc::new("[%F %H:%M:%S]".to_owned());
    let file_layer = subscriber::fmt::layer()
      .with_timer(timer.clone())
      .with_ansi(true)
      .with_target(false)
      .with_file(false)
      .with_line_number(false)
      .json()
      .with_writer(f);
    let base_layer = subscriber::fmt::layer()
      .with_timer(timer)
      .with_ansi(false)
      .with_target(false)
      .with_writer(f2);
    let def = Registry::default()
      .with(level.to_level_filter())
      .with(base_layer)
      .with(file_layer);
    (def, vec![guard, guard2])
  }
}

pub mod log {
  use std::sync::RwLockWriteGuard;

  use e_log::preload::*;
  use e_utils::{
    dialog,
    parse::{MyParseFormat as _, ParseResultDefault},
  };

  use super::Tag;
  use crate::{
    data::{LOG_HISTORY, MAX_LOG_HISTORY_CACHE},
    res::PanicAny,
  };
  /// 清理
  fn clean_up_fifo<'a, T>(guard: &mut RwLockWriteGuard<'a, Vec<T>>) {
    if guard.len() > MAX_LOG_HISTORY_CACHE {
      let end = (MAX_LOG_HISTORY_CACHE - 50) as usize;
      guard.drain(0..end); // 移除前to_remove个元素
    }
  }
  /// 添加LOG
  fn add_log(level: Level, msg: impl Into<String>, tag: Tag) {
    let msg = msg.into();
    let tag = tag.to_string();
    match level {
      Level::Off => (),
      Level::Error => error2!(tag, msg),
      Level::Warn => warn2!(tag, msg),
      Level::Info => info2!(tag, msg),
      Level::Debug => debug2!(tag, msg),
      Level::Trace => trace2!(tag, msg),
    }
    let mut ptr = LOG_HISTORY.write().panic("add log");
    clean_up_fifo(&mut ptr);
    ptr.push(("[{date} {time}]".parse_format().def(), level, msg));
  }
  /// 获取
  pub fn list(start: usize) -> Vec<(String, Level, String)> {
    let ref ptr = LOG_HISTORY.read().panic("log list clone");
    ptr.get(start..).def().to_vec()
  }
  /// len
  pub fn len() -> usize {
    let ref ptr = LOG_HISTORY.read().panic("log len");
    ptr.len()
  }
  /// clean all
  pub fn clean_all() {
    let mut ptr = LOG_HISTORY.write().panic("log clean all");
    unsafe { ptr.set_len(0) }
  }
  /// Error
  pub fn error(msg: impl Into<String>, tag: Tag) {
    add_log(Level::Error, msg, tag)
  }
  /// Warn
  pub fn warn(msg: impl Into<String>, tag: Tag) {
    add_log(Level::Warn, msg, tag)
  }
  /// Info
  pub fn info(msg: impl Into<String>, tag: Tag) {
    add_log(Level::Info, msg, tag)
  }
  /// Debug
  pub fn debug(msg: impl Into<String>, tag: Tag) {
    add_log(Level::Debug, msg, tag)
  }
  /// Trace
  pub fn trace(msg: impl Into<String>, tag: Tag) {
    add_log(Level::Trace, msg, tag)
  }
  /// error
  pub fn error_box(title: &str, msg: impl Into<String>, tag: Tag) {
    let msg = msg.into();
    error(&msg, tag);
    dialog::sync::error(title, msg);
  }
  /// error
  pub async fn a_error_box(title: &str, msg: impl Into<String>, tag: Tag) {
    let msg = msg.into();
    error(&msg, tag);
    dialog::a_sync::error(title, msg).await
  }

  /// info
  pub fn info_box(title: &str, msg: impl Into<String>, tag: Tag) {
    let msg = msg.into();
    info(&msg, tag);
    dialog::sync::info(title, msg);
  }
  /// info
  pub async fn a_info_box(title: &str, msg: impl Into<String>, tag: Tag) {
    let msg = msg.into();
    info(&msg, tag);
    dialog::a_sync::info(title, msg).await
  }

  /// warn
  pub fn warn_box(title: &str, msg: impl Into<String>, tag: Tag) {
    let msg = msg.into();
    warn(&msg, tag);
    dialog::sync::warn(title, msg);
  }
  /// warn
  pub async fn a_warn_box(title: &str, msg: impl Into<String>, tag: Tag) {
    let msg = msg.into();
    warn(&msg, tag);
    dialog::a_sync::warn(title, msg).await
  }

  /// OK 警告确认 [INFO]
  pub fn ok_zh_box(title: &str, msg: impl Into<String>, tag: Tag) {
    let msg = msg.into();
    debug(&msg, tag);
    dialog::sync::ok_zh(title, msg)
  }
  /// OK 警告确认 [INFO]
  pub async fn a_ok_zh_box(title: &str, msg: impl Into<String>, tag: Tag) {
    let msg = msg.into();
    debug(&msg, tag);
    dialog::a_sync::ok_zh(title, msg).await
  }

  /// OK [INFO]
  pub fn ok_box(title: &str, msg: impl Into<String>, tag: Tag) {
    let msg = msg.into();
    debug(&msg, tag);
    dialog::sync::ok(title, msg)
  }
  /// OK [INFO]
  pub async fn a_ok_box(title: &str, msg: impl Into<String>, tag: Tag) {
    let msg = msg.into();
    debug(&msg, tag);
    dialog::a_sync::ok(title, msg).await
  }

  /// 同意取消 [INFO]
  pub fn yesno_zh_box(title: &str, msg: impl Into<String>, tag: Tag) -> bool {
    let msg = msg.into();
    debug(&msg, tag);
    dialog::sync::yesno_zh(title, msg)
  }
  /// 同意取消 [INFO]
  pub async fn a_yesno_zh_box(title: &str, msg: impl Into<String>, tag: Tag) -> bool {
    let msg = msg.into();
    debug(&msg, tag);
    dialog::a_sync::yesno_zh(title, msg).await
  }
  /// YES NO [INFO]
  pub fn yesno_box(title: &str, msg: impl Into<String>, tag: Tag) -> bool {
    let msg = msg.into();
    debug(&msg, tag);
    dialog::sync::yesno(title, msg)
  }
  /// YES NO [INFO]
  pub async fn a_yesno_box(title: &str, msg: impl Into<String>, tag: Tag) -> bool {
    let msg = msg.into();
    debug(&msg, tag);
    dialog::a_sync::yesno(title, msg).await
  }
}

#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Tag {
  Dev,
  Login,
  AutoHandle,
  DataAnalysis,
  DataPrint,
  Window,
  LoadSetting,
  SaveSetting,
  DatabaseOffline,
  #[default]
  Unknow,
}

impl Display for Tag {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      Tag::Login => "登录",
      Tag::Window => "窗口",
      Tag::LoadSetting => "加载配置",
      Tag::SaveSetting => "保存配置",
      Tag::AutoHandle => "自动监听",
      Tag::DatabaseOffline => "离线数据库",
      Tag::DataAnalysis => "数据分析",
      Tag::DataPrint => "数据打印",
      Tag::Unknow => "未知",
      Tag::Dev => "BUG",
    };
    write!(f, "{}", s)
  }
}

impl Into<Value> for Tag {
  fn into(self) -> Value {
    match serde_json::to_string(&self) {
      Ok(s) => Value::String(s),
      Err(_) => Value::Null,
    }
  }
}
