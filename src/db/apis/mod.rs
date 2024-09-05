use crate::config::Config;
use log::{select_log, update_log};
use sqlx::SqlitePool;
use user::{
  select_user, select_user_cfg, select_user_info, update_user, update_user_cfg, update_user_info,
  update_user_with_passwd,
};
pub mod app;
pub mod log;
pub mod user;
/// sqlite3
pub async fn load_all_data(pool: &SqlitePool, id: i64) -> Result<Config, String> {
  async fn f(pool: &SqlitePool, id: i64) -> sqlx::Result<Config> {
    let mut slf = Config::default();
    slf.user = select_user(pool, id).await?;
    slf.user_cfg = select_user_cfg(pool, id).await?;
    slf.user_info = select_user_info(pool, id).await?;
    slf.log = select_log(pool, id).await?;
    Ok(slf)
  }
  f(pool, id).await.map_err(|e| e.to_string())
}

/// sqlite3
pub async fn save_all_data(pool: &SqlitePool, slf: Config) -> Result<Config, String> {
  async fn f(pool: &SqlitePool, slf: Config) -> sqlx::Result<Config> {
    if slf.user.password.is_empty() {
      update_user(pool, &slf.user).await?;
    } else {
      update_user_with_passwd(pool, slf.user.clone()).await?;
    }
    update_user_info(pool, &slf.user_info).await?;
    update_user_cfg(pool, slf.user_cfg.clone()).await?;
    update_log(pool, slf.log.clone()).await?;
    Ok(slf)
  }
  f(pool, slf).await.map_err(|e| e.to_string())
}
