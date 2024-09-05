//! Data components of MyApp
use std::{
  collections::HashMap,
  env,
  path::PathBuf,
  sync::{Arc, RwLock},
};

use e_log::Level;
use e_utils::{
  once_cell::sync::Lazy,
  regex::{Regex, REGEX_MAC},
};
use iced::widget::image;

use crate::{db::Db, res::PanicAny as _};

pub mod font;
pub mod icon;

/// 最大日志缓存
pub const MAX_LOG_HISTORY_CACHE: usize = 500;
/// 初始化参数
pub static ORIGIN: Lazy<PathBuf> = Lazy::new(|| env::current_dir().expect("ORIGIN"));

pub static INIT_FONT: &[u8] = include_bytes!("../../assets/font/AlimamaDongFangDaKai-Regular.ttf");
pub static ICON_FONT: &[u8] = include_bytes!("../../assets/font/iconfont.ttf");

/// 标题LOGO
pub static TITLE_LOGO: &[u8] = include_bytes!("../../assets/img/logos/title.png");
/// 主LOGO
pub static MAIN_LOGO: &[u8] = include_bytes!("../../assets/img/logos/icon.png");

// 定义一个静态正则表达式，用于匹配MAC地址
// pub static MAC_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(REGEX_MAC).panic("MAC_REGEX"));

pub static IMG_MAP: Lazy<HashMap<&'static str, image::Handle>> = Lazy::new(|| {
  let load = |bytes: &'static [u8]| image::Handle::from_memory(bytes);

  HashMap::from([("main", load(MAIN_LOGO)), ("title", load(TITLE_LOGO))])
});
// Define a global static variable that will be initialized lazily.
pub static LOG_HISTORY: Lazy<Arc<RwLock<Vec<(String, Level, String)>>>> =
  Lazy::new(|| Arc::new(RwLock::new(Vec::new())));

// Define a global static variable that will be initialized lazily.
pub static DB_SQLITE: Lazy<Arc<tokio::sync::RwLock<Db>>> = Lazy::new(|| {
  Arc::new(tokio::sync::RwLock::new({
    let migrate_workspace = ORIGIN.join("migrations").join("sqlite");
    let url = String::from("sqlite://app.db");
    Db::new(url, migrate_workspace)
  }))
});
