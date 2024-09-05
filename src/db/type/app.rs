use e_utils::time::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(sqlx::FromRow)]
pub struct ExtendAppQ {
  pub id: i32,
  pub tag: String,
  pub label: String,
  pub enable: bool,
  pub r#type: i32,
  pub priority: i32,
  pub is_check: bool,
  pub is_repeat: bool,
  pub is_wait: bool,
  pub timeout: i32,
  pub count: i32,
  pub cmd: Value,
  pub cwd: Option<String>,
  pub res_url: Option<String>,
  pub filter: Value,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExtendApp {
  pub id: i32,
  pub tag: String,
  pub label: String,
  pub enable: bool,
  pub r#type: i32,
  pub priority: i32,
  pub is_check: bool,
  pub is_repeat: bool,
  pub is_wait: bool,
  pub timeout: i32,
  pub count: i32,
  pub cmd: Vec<String>,
  pub cwd: Option<String>,
  pub res_url: Option<String>,
  pub filter: Vec<String>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl From<ExtendAppQ> for ExtendApp {
  fn from(value: ExtendAppQ) -> Self {
    Self {
      id: value.id,
      tag: value.tag,
      label: value.label,
      enable: value.enable,
      r#type: value.r#type,
      priority: value.priority,
      is_check: value.is_check,
      is_repeat: value.is_repeat,
      is_wait: value.is_wait,
      timeout: value.timeout,
      count: value.count,
      cmd: value
        .cmd
        .as_array()
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default(),
      cwd: value.cwd,
      res_url: value.res_url,
      filter: value
        .filter
        .as_array()
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default(),
      created_at: value.created_at,
      updated_at: value.updated_at,
    }
  }
}
impl From<ExtendApp> for ExtendAppQ {
  fn from(value: ExtendApp) -> Self {
    Self {
      id: value.id,
      tag: value.tag,
      label: value.label,
      enable: value.enable,
      r#type: value.r#type,
      priority: value.priority,
      is_check: value.is_check,
      is_repeat: value.is_repeat,
      is_wait: value.is_wait,
      timeout: value.timeout,
      count: value.count,
      cmd: serde_json::Value::Array(value.cmd.into_iter().map(serde_json::Value::String).collect()),
      cwd: value.cwd,
      res_url: value.res_url,
      filter: serde_json::Value::Array(value.filter.into_iter().map(serde_json::Value::String).collect()),
      created_at: value.created_at,
      updated_at: value.updated_at,
    }
  }
}
