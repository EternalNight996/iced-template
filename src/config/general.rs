use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct GeneralConfig {
  pub title: String,
  pub version: String,
}

impl Default for GeneralConfig {
  fn default() -> Self {
    Self {
      title: String::new(),
      version: String::new(),
    }
  }
}
