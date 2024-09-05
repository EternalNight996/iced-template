pub mod apis;
pub mod sqlite;
pub mod r#type;
use std::path::Path;
/// database manager
use std::{ffi::OsStr, path::PathBuf};

use e_utils::async_runtime::block_on;
pub use sqlite::*;
use sqlx::{migrate::Migrator, query, Error as SqlxError, Result as SqlxResult, Row};

use crate::config::logger::{log, Tag};
/// A trait which provides connection-specific functionality.
pub trait ManageConn: Send + Sync + 'static {
  /// The connection type this manager deals with.
  type Connection: Send + 'static;
  /// The error type returned by `Connection`s.
  type Error: std::error::Error + 'static;
  /// Attempts to create a new connection.
  fn pool(&self) -> Self::Connection;
}

/// Migrations table type
#[derive(Debug)]
pub enum DatabaseTableType {
  /// all
  All,
  /// last
  Last,
  /// num(i64)
  Num(i64),
  /// none
  None,
}
impl ToString for DatabaseTableType {
  fn to_string(&self) -> String {
    String::from(match &*self {
      DatabaseTableType::All => "all",
      DatabaseTableType::Last => "last",
      DatabaseTableType::Num(_) => "num",
      DatabaseTableType::None => "",
    })
  }
}
impl DatabaseTableType {
  /// index position
  pub fn index(&self) -> i64 {
    match self {
      DatabaseTableType::All => 0,
      DatabaseTableType::Last => 0,
      DatabaseTableType::Num(i) => *i,
      DatabaseTableType::None => 99999,
    }
  }

  /// match table
  pub fn match_table(key: &str) -> Self {
    let key = key.to_lowercase();
    if key == DatabaseTableType::All.to_string() {
      DatabaseTableType::All
    } else if key == DatabaseTableType::Last.to_string() {
      DatabaseTableType::Last
    } else if key.parse::<i64>().is_ok() {
      DatabaseTableType::Num(key.parse::<i64>().unwrap_or(99999))
    } else {
      DatabaseTableType::None
    }
  }
}

/// run a revert SQL from migration in a DDL executor
/// deletes the row in [_migrations] table with specified migration version on completion (success or failure)
/// returns the time taking to run the migration SQL
#[allow(unused)]
pub async fn migrate_revert(
  migrate_workspace: &str,
  pool: &DbPool,
  table_type: DatabaseTableType,
) -> SqlxResult<()> {
  // Danger! will clean all table from _sqlx_migrations!
  if let DatabaseTableType::Last = table_type {
    let count = query("SELECT COUNT(*) as count FROM _sqlx_migrations")
      .fetch_one(pool)
      .await?
      .get::<i64, &str>("count");
    get_migrator(migrate_workspace).await?.undo(pool, count - 1).await?;
    Ok(())
  } else if let DatabaseTableType::None = table_type {
    Err(SqlxError::TypeNotFound {
      type_name: table_type.to_string(),
    })
  } else {
    Ok(
      get_migrator(migrate_workspace)
        .await?
        .undo(pool, table_type.index())
        .await?,
    )
  }
}
/// Database migrate service
pub async fn migrate_run(migrate_workspace: &str, db_pool: &DbPool) -> SqlxResult<()> {
  Ok(
    get_migrator(migrate_workspace)
      .await?
      .run(&mut db_pool.acquire().await?)
      .await?,
  )
}
async fn get_migrator<S: AsRef<OsStr> + ?Sized>(migrate_workspace: &S) -> SqlxResult<Migrator> {
  Ok(Migrator::new(Path::new(migrate_workspace)).await?)
}

/// DB API
#[derive(Default)]
pub struct Db {
  inner: Option<DbClient>,
  migrate_workspace: PathBuf,
  url: String,
}
impl Db {
  pub fn new(url: impl Into<String>, migrate_workspace: impl AsRef<Path>) -> Self {
    let mut slf = Self::default();
    slf.migrate_workspace = migrate_workspace.as_ref().to_path_buf();
    slf.url = url.into();
    slf
  }

  /// local db
  pub fn pool(&self) -> DbPool {
    if let Some(x) = &self.inner {
      x.pool()
    } else {
      panic!("解包DB失败")
    }
  }

  /// 连接数据库
  pub async fn a_connect(&mut self) -> e_utils::Result<()> {
    match DbClient::init_conn(&self.url, &self.migrate_workspace.to_string_lossy()).await {
      Ok(x) => {
        self.inner = Some(x);
        Ok(())
      }
      Err(e) => {
        log::error(format!("{e}"), Tag::LoadSetting);
        Err(e.to_string().into())
      }
    }
  }

  /// 连接数据库
  #[allow(unused)]
  pub fn connect(&mut self) -> e_utils::Result<()> {
    block_on(self.a_connect())
  }
}
