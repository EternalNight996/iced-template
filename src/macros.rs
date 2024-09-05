macro_rules! build_panic_any {
  ($target:path) => {
    impl<T> PanicAny<T> for $target {
      fn panic(self, msg: impl AsRef<str>) -> T {
        match self {
          Ok(x) => x,
          Err(e) => {
            panic!("{}: {e}", msg.as_ref());
          }
        }
      }
    }
  };
}

macro_rules! a_task {
  ($target:expr) => {
    return iced::Command::perform($target, |_| Message::Ignore)
  };
}
