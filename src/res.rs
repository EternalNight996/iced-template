use e_utils::http::status::StatusCode;
use std::borrow::Cow;

/// web result
pub type Result<T> = std::result::Result<T, Error>;
/// Http share error
#[derive(thiserror::Error, Debug)]
pub enum Error {
  /// argon2
  #[error("Argon2: {0}")]
  Argon2(#[from] argon2::Error),
  /// Iced
  #[error("Iced: ")]
  Iced(#[from] iced::Error),
  /// 数据库
  #[error("Database Sqlx: {0}")]
  Database(#[from] sqlx::Error),
  /// 数据解析
  #[error("Data parse: {0}")]
  DataParse(String),
  /// CEUtils
  #[error("CEUtils: {0}")]
  CEUtils(#[from] e_utils::CError),
  /// EUtils
  #[error("EUtils: {0}")]
  EUtils(#[from] e_utils::Error),
  /// BadRequest `400 Bad Request`
  #[error("400 拒绝请求")]
  BadRequest,
  /// 已存在
  #[error("Exists: {0}")]
  Exists(Cow<'static, str>),
  /// Return `401 Unauthorized`
  #[error("authentication required: {0}")]
  Unauthorized(&'static str),
  /// Return `403 Forbidden`
  #[error("user may not perform that action")]
  Forbidden,
  /// Return `404 Not Found`
  #[error("request path not found: {0}")]
  NotFound(Cow<'static, str>),
  /// 不支持
  #[error("Not support: {0}")]
  Unsupport(Cow<'static, str>),
  ///
  #[error("Empty")]
  Empty,
  /// Return `502 Internal Server Error` on an `anyhow::Error`.
  #[error("Log: {0}")]
  Log(Cow<'static, str>),
}
impl Error {
  /// 状态码获取
  pub fn code(&self) -> String {
    match self {
      Self::BadRequest
      | Self::DataParse(_)
      | Self::Log(_)
      | Self::Exists(_)
      | Self::Empty
      | Self::Iced(_)
      | Self::Unsupport(_)
      | Self::CEUtils(_)
      | Self::Argon2(_)
      | Self::EUtils(_) => StatusCode::BAD_REQUEST.to_string(),
      Self::Unauthorized(_) => StatusCode::UNAUTHORIZED.to_string(),
      Self::Forbidden => StatusCode::FORBIDDEN.to_string(),
      Self::Database(derr) => derr
        .as_database_error()
        .and_then(|x| x.code().and_then(|x| Some(x.to_string())))
        .unwrap_or(StatusCode::DATABASE_INIT.to_string()),
      Self::NotFound(_) => StatusCode::NOT_FOUND.to_string(),
    }
  }
}

pub trait PanicAny<T> {
  fn panic(self, msg: impl AsRef<str>) -> T;
}

build_panic_any!(crate::Result<T>);
build_panic_any!(e_utils::Result<T>);
build_panic_any!(sqlx::Result<T>);
build_panic_any!(std::result::Result<T, toml::ser::Error>);
build_panic_any!(std::result::Result<T, toml::de::Error>);
build_panic_any!(std::result::Result<T, e_utils::regex::Error>);
build_panic_any!(std::result::Result<T, String>);
build_panic_any!(std::result::Result<T, std::sync::PoisonError<T>>);
