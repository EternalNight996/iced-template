use iced::{font, Command};

pub fn load() -> Command<Result<(), font::Error>> {
  Command::batch([font::load(super::INIT_FONT), font::load(super::ICON_FONT)])
}
