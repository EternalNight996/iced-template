use sqlx::{query_as, query_scalar, sqlite::SqliteQueryResult, SqlitePool};

use crate::db::r#type::user::{User, UserCfg, UserCfgQ, UserInfo};
/// 获取用户
pub async fn select_user(pool: &SqlitePool, id: i64) -> sqlx::Result<User> {
  let mut user = query_as::<_, User>("SELECT * from  user WHERE (id=?)")
    .bind(id)
    .fetch_one(pool)
    .await?;
  user.password = "".to_string();
  Ok(user)
}
/// 获取用户配置
pub async fn select_user_cfg(pool: &SqlitePool, id: i64) -> sqlx::Result<UserCfg> {
  query_as::<_, UserCfgQ>("SELECT * from  user_cfg WHERE (user_id=?)")
    .bind(id)
    .fetch_one(pool)
    .await
    .map(UserCfg::from)
}
/// 获取用户信息
pub async fn select_user_info(pool: &SqlitePool, id: i64) -> sqlx::Result<UserInfo> {
  query_as::<_, UserInfo>("SELECT * from  user_info WHERE (user_id=?)")
    .bind(id)
    .fetch_one(pool)
    .await
}
/// 更新用户配置
pub async fn update_user_cfg(pool: &SqlitePool, v: UserCfg) -> sqlx::Result<SqliteQueryResult> {
  let v = UserCfgQ::from(v);
  let res = sqlx::query(
    r#"
        UPDATE user_cfg 
        SET height = ?1, width = ?2, default_text_size = ?3, theme = ?4, 
            resizable = ?5, decorations = ?6, transparent = ?7, 
            highline_theme = ?8, updated_at = CURRENT_TIMESTAMP 
        WHERE user_id = ?9
        "#,
  )
  .bind(v.height)
  .bind(v.width)
  .bind(v.default_text_size)
  .bind(v.theme)
  .bind(v.resizable)
  .bind(v.decorations)
  .bind(v.transparent)
  .bind(v.highline_theme)
  .bind(v.user_id)
  .execute(pool)
  .await?;
  Ok(res)
}
/// 更新用户信息
pub async fn update_user_info(pool: &SqlitePool, v: &UserInfo) -> sqlx::Result<SqliteQueryResult> {
  let res = sqlx::query(
    r#"
        UPDATE user_info
        SET nickname = ?1, avatar_url = ?2, description = ?3, identity = ?4, 
            status = ?5, updated_at = CURRENT_TIMESTAMP 
        WHERE user_id = ?6
        "#,
  )
  .bind(&v.nickname)
  .bind(&v.avatar_url)
  .bind(&v.description)
  .bind(&v.identity)
  .bind(v.status)
  .bind(v.user_id)
  .execute(pool)
  .await?;
  Ok(res)
}
/// 更新用户
pub async fn update_user_with_passwd(pool: &SqlitePool, mut v: User) -> sqlx::Result<SqliteQueryResult> {
  v.password = create_password_hash(v.password.as_bytes())
    .map_err(|e| sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())))?;
  let res = sqlx::query(
    r#"
        UPDATE user 
        SET name = ?1, email = ?2, prefix_mobile = ?3, mobile = ?4, 
            password = ?5, updated_at = CURRENT_TIMESTAMP 
        WHERE id = ?6
        "#,
  )
  .bind(&v.name)
  .bind(&v.email)
  .bind(&v.prefix_mobile)
  .bind(&v.mobile)
  .bind(&v.password)
  .bind(v.id)
  .execute(pool)
  .await?;
  Ok(res)
}
/// 更新用户
pub async fn update_user(pool: &SqlitePool, v: &User) -> sqlx::Result<SqliteQueryResult> {
  let res = sqlx::query(
    r#"
        UPDATE user 
        SET email = ?1, prefix_mobile = ?2, mobile = ?3,  updated_at = CURRENT_TIMESTAMP 
        WHERE id = ?4
        "#,
  )
  .bind(&v.email)
  .bind(&v.prefix_mobile)
  .bind(&v.mobile)
  .bind(v.id)
  .execute(pool)
  .await?;
  Ok(res)
}

/// 校验身份
pub async fn verify_password(pool: &SqlitePool, username: &str, passwd: &str) -> crate::Result<bool> {
  let user_passwd: String = query_scalar("SELECT password FROM user WHERE name = ?")
    .bind(username)
    .fetch_one(pool)
    .await?;
  Ok(argon2::verify_encoded(&user_passwd, passwd.as_bytes())?)
}

/// 校验身份
pub fn create_password_hash(passwd: &[u8]) -> crate::Result<String> {
  let config = argon2::Config {
    variant: argon2::Variant::Argon2i,
    version: argon2::Version::Version13,
    mem_cost: 1024, // 降低内存成本
    time_cost: 2,   // 降低时间成本
    lanes: 16,      // 增加并行度
    secret: &[],
    ad: &[],
    hash_length: 16, // 减少哈希长度
  };
  let hash = argon2::hash_encoded(passwd, b"eternalnight", &config)?;
  Ok(hash)
}
