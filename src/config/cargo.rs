use serde::{Deserialize, Serialize};

use crate::res::PanicAny as _;
/// Cargo Config
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
  pub package: Package,
}
/// Cargo Package
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Package {
  pub name: String,
  pub authors: Vec<String>,
  pub description: String,
  pub version: String,
  pub edition: String,
  pub build: String,
  pub license: Option<String>,
  pub license_file: Option<String>,
  pub repository: String,
}
impl Default for Config {
  fn default() -> Self {
    let cargo_bytes = include_bytes!("../../Cargo.toml");
    let cargo_str = String::from_utf8_lossy(cargo_bytes);
    let conf: Config = toml::from_str(&cargo_str).panic("无法解析Cargo.toml");
    conf
  }
}
