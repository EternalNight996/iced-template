use sqlx::{query_as, sqlite::SqliteQueryResult, SqlitePool};

use crate::{config::logger::LogCfg, db::r#type::logger::LogQ};
/// 获取数据分析配置
pub async fn select_log(pool: &SqlitePool, id: i64) -> sqlx::Result<LogCfg> {
  query_as::<_, LogQ>("SELECT * from  log WHERE (id=?)")
    .bind(id)
    .fetch_one(pool)
    .await
    .map(LogCfg::from)
}

/// 更新用户配置
pub async fn update_log(pool: &SqlitePool, cfg: LogCfg) -> sqlx::Result<SqliteQueryResult> {
  let v: LogQ = cfg.into();
  let res = sqlx::query(
    r#"
        UPDATE log
        SET level = ?1, folder = ?2, fname = ?3, format = ?4, 
            output_list = ?5, tracing = ?6, updated_at = CURRENT_TIMESTAMP
        WHERE id = ?7
        "#,
  )
  .bind(v.level)
  .bind(&v.folder)
  .bind(&v.fname)
  .bind(&v.format)
  .bind(&v.output_list)
  .bind(v.tracing)
  .bind(v.id)
  .execute(pool)
  .await?;
  Ok(res)
}
