#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(clippy::needless_lifetimes)]
#![allow(
  clippy::cognitive_complexity,
  clippy::large_enum_variant,
  clippy::module_inception,
  clippy::needless_doctest_main
)]
#![warn(missing_debug_implementations, rust_2018_idioms)]
#![deny(unused_must_use)]
#![doc(test(
  no_crate_inject,
  attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

#[macro_use]
#[path = "macros.rs"]
mod macros;
mod config;
mod data;
mod db;
mod event;
mod plugins;
mod res;
mod ui;
mod utils;
pub(crate) use res::Result;
use ui::main_app;

// #[tokio::main(flavor = "multi_thread", worker_threads = 10)]
fn main() -> Result<()> {
  let _ = main_app::App::launch()?;
  Ok(())
}
