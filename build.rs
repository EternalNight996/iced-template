fn main() {
  #[cfg(windows)]
  {
    static_vcruntime::metabuild();

    winresource::WindowsResource::new()
      .set_icon("./scripts/windows/icon.ico")
      .set_manifest(include_str!("./scripts/windows/app.manifest"))
      .compile()
      .expect("embed Windows resources");
  }

  #[cfg(feature = "built")]
  built::write_built_file().expect("Failed to acquire build-time information");
}
