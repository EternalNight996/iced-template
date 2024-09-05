use e_utils::time::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone, Default, sqlx::FromRow)]
pub struct LogQ {
  pub id: i32,
  pub level: i32,
  pub folder: String,
  pub fname: String,
  pub format: Option<String>,
  pub output_list: Value,
  pub tracing: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
