use e_utils::time::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::config::theme::Themes;

#[derive(Debug, Clone, Default, sqlx::FromRow)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub email: Option<String>,
  pub prefix_mobile: Option<String>,
  pub mobile: Option<String>,
  pub password: String,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
#[derive(Debug, Clone, Default, sqlx::FromRow)]
pub struct UserInfo {
  pub user_id: i32,
  pub nickname: Option<String>,
  pub avatar_url: Option<String>,
  pub description: Option<String>,
  pub identity: Value,
  pub status: bool,
  pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserCfg {
  pub user_id: i32,
  pub height: f32,
  pub width: f32,
  pub default_text_size: f32,
  pub theme: Themes,
  pub highline_theme: i32,
  pub resizable: bool,
  /// Whether the window should have a border, a title bar, etc. or not.
  pub decorations: bool,
  /// Whether the window should be transparent.
  pub transparent: bool,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, sqlx::FromRow)]
pub struct UserCfgQ {
  pub user_id: i32,
  pub height: f32,
  pub width: f32,
  pub default_text_size: f32,
  pub theme: i32,
  pub highline_theme: i32,
  pub resizable: bool,
  /// Whether the window should have a border, a title bar, etc. or not.
  pub decorations: bool,
  /// Whether the window should be transparent.
  pub transparent: bool,
  pub updated_at: DateTime<Utc>,
}

impl From<UserCfgQ> for UserCfg {
  fn from(q: UserCfgQ) -> Self {
    UserCfg {
      user_id: q.user_id,
      height: q.height,
      width: q.width,
      default_text_size: q.default_text_size,
      theme: Themes::from(q.theme),
      highline_theme: q.highline_theme,
      resizable: q.resizable,
      decorations: q.decorations,
      transparent: q.transparent,
      updated_at: q.updated_at,
    }
  }
}

impl From<UserCfg> for UserCfgQ {
  fn from(q: UserCfg) -> Self {
    Self {
      user_id: q.user_id,
      height: q.height,
      width: q.width,
      default_text_size: q.default_text_size,
      theme: q.theme as i32,
      highline_theme: q.highline_theme,
      resizable: q.resizable,
      decorations: q.decorations,
      transparent: q.transparent,
      updated_at: q.updated_at,
    }
  }
}
