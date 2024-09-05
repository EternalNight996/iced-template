/// database manager
use std::{str::FromStr as _, time::Duration};

use sqlx::{
  pool::PoolConnection,
  query,
  sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions},
  ConnectOptions as _, Connection as _, Error, Row, Sqlite,
};

use super::{migrate_run, ManageConn};
use crate::{
  config::logger::{log, Tag},
  res::Result,
};
// use super::EManageConnectionT;
const SQLITE_MAX_CONNECTIONS: u32 = 30;
const SQLITE_MAX_LINFETIME: Duration = Duration::from_secs(1800);
const SQLITE_IDLE_TIMEOUT: Duration = Duration::from_secs(600);

pub type DbPool = SqlitePool;
/// Share model
#[derive(Debug, Clone)]
pub struct DbClient(DbPool);

/// Pgsql Connection
#[allow(unused)]
#[derive(Debug)]
pub struct DbConn(PoolConnection<Sqlite>);

impl DbClient {
  /// 搭建
  pub async fn init(self, migrate_workspace: &str) -> Result<Self> {
    println!("{migrate_workspace}");
    // create database if it does not exist
    let ref pool = self.pool();
    match query("SELECT COUNT(*) as count FROM _sqlx_migrations")
      .fetch_one(pool)
      .await
    {
      Ok(x) => {
        let count = x.get::<i64, &str>("count");
        if count < 2 {
          migrate_run(migrate_workspace, pool).await?;
        }
      }
      Err(e) => {
        log::warn(e.to_string(), Tag::DatabaseOffline);
        migrate_run(migrate_workspace, pool).await?
      }
    };
    Ok(self)
  }

  /// # Connection
  /// | URL | Description |
  /// | -- | -- |
  /// `sqlite::memory:` | Open an in-memory database. |
  /// `sqlite:data.db` | Open the file `data.db` in the current directory. |
  /// `sqlite://data.db` | Open the file `data.db` in the current directory. |
  /// `sqlite:///data.db` | Open the file `data.db` from the root (`/`) directory. |
  /// `sqlite://data.db?mode=ro` | Open the file `data.db` for read-only access. |
  pub async fn connect(url: &str) -> Result<Self> {
    SqliteConnectOptions::from_str(url)?
      .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
      .create_if_missing(true)
      .connect()
      .await?
      .close()
      .await?;
    let pool = SqlitePoolOptions::new()
      .max_connections(SQLITE_MAX_CONNECTIONS)
      .idle_timeout(SQLITE_IDLE_TIMEOUT)
      .max_lifetime(SQLITE_MAX_LINFETIME)
      .connect(url)
      .await?;
    log::debug(format!("Connected SqliteSql database URL: {url}"), Tag::DatabaseOffline);
    Ok(Self(pool))
  }

  /// 连接并初始化
  pub async fn init_conn(url: &str, migrate_workspace: impl AsRef<str>) -> Result<Self> {
    let conn = Self::connect(url).await?.init(migrate_workspace.as_ref()).await?;
    Ok(conn)
  }
}

impl ManageConn for DbClient {
  type Connection = SqlitePool;
  type Error = Error;

  fn pool(&self) -> Self::Connection {
    self.0.clone()
  }
}
