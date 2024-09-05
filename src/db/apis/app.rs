use crate::db::r#type::app::{ExtendApp, ExtendAppQ};
use sqlx::{query_as, query_scalar, SqlitePool};

/// 插入扩展应用数据
pub async fn insert_extend_app(pool: &SqlitePool, app: ExtendApp) -> sqlx::Result<i64> {
  let app = ExtendAppQ::from(app);
  let res = sqlx::query(
        r#"
        INSERT INTO externApp (tag, label, enable, type, priority, is_check, is_repeat, is_wait, timeout, count, cmd, cwd, res_url, filter)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
        "#,
    )
    .bind(app.tag)
    .bind(app.label)
    .bind(app.enable)
    .bind(app.r#type)
    .bind(app.priority)
    .bind(app.is_check)
    .bind(app.is_repeat)
    .bind(app.is_wait)
    .bind(app.timeout)
    .bind(app.count)
    .bind(app.cmd)
    .bind(app.cwd)
    .bind(app.res_url)
    .bind(app.filter)
    .execute(pool)
    .await?;

  Ok(res.last_insert_rowid())
}

/// 获取扩展应用数据
pub async fn select_extend_app(pool: &SqlitePool, id: i32) -> sqlx::Result<ExtendApp> {
  query_as::<_, ExtendAppQ>("SELECT * FROM externApp WHERE id = ?")
    .bind(id)
    .fetch_one(pool)
    .await
    .map(ExtendApp::from)
}

/// 更新扩展应用数据
pub async fn update_extend_app(pool: &SqlitePool, app: ExtendApp) -> sqlx::Result<i64> {
  let app = ExtendAppQ::from(app);
  let res = sqlx::query(
        r#"
        UPDATE externApp
        SET tag = ?1, label = ?2, enable = ?3, type = ?4, priority = ?5, is_check = ?6, is_repeat = ?7, is_wait = ?8, 
            timeout = ?9, count = ?10, cmd = ?11, cwd = ?12, res_url = ?13, filter = ?14, updated_at = CURRENT_TIMESTAMP
        WHERE id = ?15
        "#,
    )
    .bind(app.tag)
    .bind(app.label)
    .bind(app.enable)
    .bind(app.r#type)
    .bind(app.priority)
    .bind(app.is_check)
    .bind(app.is_repeat)
    .bind(app.is_wait)
    .bind(app.timeout)
    .bind(app.count)
    .bind(app.cmd)
    .bind(app.cwd)
    .bind(app.res_url)
    .bind(app.filter)
    .bind(app.id)
    .execute(pool)
    .await?;

  Ok(res.rows_affected() as i64)
}

/// 删除扩展应用数据
pub async fn delete_extend_app(pool: &SqlitePool, id: i32) -> sqlx::Result<i64> {
  let res = sqlx::query("DELETE FROM externApp WHERE id = ?")
    .bind(id)
    .execute(pool)
    .await?;

  Ok(res.rows_affected() as i64)
}

/// 判断扩展应用是否存在
pub async fn exist_extend_app(pool: &SqlitePool, id: i32) -> sqlx::Result<bool> {
  let res: bool = query_scalar("SELECT 1 FROM externApp WHERE id = ?")
    .bind(id)
    .fetch_one(pool)
    .await?;

  Ok(res)
}

/// 获取所有扩展应用数据
pub async fn select_all_extend_apps(pool: &SqlitePool) -> sqlx::Result<Vec<ExtendApp>> {
  query_as::<_, ExtendAppQ>("SELECT * FROM externApp")
    .fetch_all(pool)
    .await
    .map(|apps| apps.into_iter().map(ExtendApp::from).collect())
}
