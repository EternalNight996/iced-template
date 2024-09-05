pub mod cargo;
pub mod general;
pub mod logger;
pub mod theme;
use e_utils::{parse::MyParseFormat as _, Result};

use crate::{
  config::logger::LogCfg,
  data::{DB_SQLITE, ORIGIN},
  db::{
    apis::load_all_data,
    r#type::user::{User, UserCfg, UserInfo},
  },
  res::PanicAny,
};

/// 全局配置
#[derive(Clone, Debug, Default)]
pub struct Config {
  pub log: LogCfg,
  pub user: User,
  pub user_info: UserInfo,
  pub user_cfg: UserCfg,
  pub cargo: cargo::Config,
}

impl Config {
  /// 初始化数据
  #[tokio::main]
  pub async fn init_base_conf(&mut self) {
    let mut db_api = DB_SQLITE.write().await;
    db_api.a_connect().await.panic("数据库Sqlite连接");
    let pool = db_api.pool();
    *self = load_all_data(&pool, 0).await.panic("加载数据");
    self.log.fname = self.rkey(&self.log.fname).panic("init_base_conf");
    self.log.folder = self.rkey(&self.log.folder).panic("init_base_conf");
  }

  /// 筛选
  pub fn rkey<S: AsRef<str>>(&self, value: S) -> Result<String> {
    let match_fn = |k: String| -> String {
      match &*k {
        "title" => self.cargo.package.description.clone(),
        "version" => self.cargo.package.version.clone(),
        "name" => self.cargo.package.name.clone(),
        "origin" => ORIGIN.to_string_lossy().to_string(),
        _ => String::new(),
      }
    };
    let res = value.as_ref().parse_format()?.parse_replace('#', '#', match_fn);
    res
  }
}
