use iced::color;
use serde::{Deserialize, Serialize};

use iced::Color;

/// 主题
#[derive(Debug)]
pub struct Theme {
  pub name: String,
  pub palette: Palette,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
  pub background: Color,
  pub middleground: Color,
  pub foreground: Color,
  pub border: Color,
  pub text: Color,
  pub main: Color,
  pub error: Color,
  pub warning: Color,
  pub success: Color,
  pub waveform: Color,
  pub black: Color,
}

impl Default for Palette {
  fn default() -> Self {
    Themes::Dark.palette()
  }
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Themes {
  #[default]
  Dark,
  Dracula,
  Catppuccin,
  Ferra,
  Nord,
  LMMS,
  OneShot,
  HighContrast,
}
impl From<i32> for Themes {
  fn from(value: i32) -> Self {
    let v = value as usize;
    if v > Self::ALL.len() {
      panic!("Themes [ {} ] > {:#?}", v, Self::ALL);
    }
    Self::ALL[v]
  }
}
impl Themes {
  pub const ALL: [Self; 8] = [
    Self::Dark,
    Self::Dracula,
    Self::Catppuccin,
    Self::Nord,
    Self::Ferra,
    Self::LMMS,
    Self::OneShot,
    Self::HighContrast,
  ];

  pub fn palette(&self) -> Palette {
    match self {
      Self::Dark => Palette {
        middleground: color!(0x272727),
        foreground: color!(0x353535),
        background: color!(0x151515),
        border: color!(0x555555),
        main: color!(0xBA84FC),
        text: color!(0xE0E0E0),
        error: color!(0xff5555),
        warning: color!(0xffcc00),
        success: color!(0x49eb7a),
        waveform: color!(0xBA84FC),
        black: color!(0x000000),
      },
      // based on: https://github.com/dracula/dracula-theme
      Self::Dracula => Palette {
        middleground: color!(0x282a36),
        foreground: color!(0x44475a),
        background: color!(0x1D1E26),
        border: color!(0x4f5263),
        main: color!(0xff79c6),
        text: color!(0xf8f8f2),
        error: color!(0xff5555),
        warning: color!(0xffcc00),
        success: color!(0x50fa7b),
        waveform: color!(0xff79c6),
        black: color!(0x000000),
      },
      Self::LMMS => Palette {
        middleground: color!(0x262B30),
        foreground: color!(0x3B424A), //3B424A
        background: color!(0x111314),
        border: color!(0x4C5864),
        main: color!(0x0BD556),
        text: color!(0xe5e9f0),
        error: color!(0xff5555),
        warning: color!(0xffcc00),
        success: color!(0x0BD556),
        waveform: color!(0xFF8F05),
        black: color!(0x000000),
      },
      // https://github.com/nordtheme/nord
      Self::Nord => Palette {
        middleground: color!(0x2e3440),
        foreground: color!(0x3b4252),
        background: color!(0x21252d),
        border: color!(0x50586d),
        main: color!(0x88c0d0),
        text: color!(0xe5e9f0),
        error: color!(0xbf616a),
        warning: color!(0xbf616a),
        success: color!(0xa3be8c),
        waveform: color!(0x88c0d0),
        black: color!(0x000000),
      },
      Self::OneShot => Palette {
        middleground: color!(0x1A0B1D),
        foreground: color!(0x2B0D1A),
        background: color!(0x100213),
        border: color!(0xba9e59),
        main: color!(0xe2bc5a), //color!(0xF48550),
        text: color!(0xFEFECD),
        error: color!(0xff5555),
        warning: color!(0xffcc00),
        success: color!(0x80FF80),
        waveform: color!(0xe2bc5a),
        black: color!(0x000000),
      },

      // based on: https://github.com/catppuccin/catppuccin
      Self::Catppuccin => Palette {
        middleground: color!(0x1E1E28),
        foreground: color!(0x332E41),
        background: color!(0x1B1923),
        border: color!(0x6E6C7E),
        main: color!(0xC6AAE8),
        text: color!(0xFEFECD),
        error: color!(0xE38C8F),
        warning: color!(0xE38C8F),
        success: color!(0xB1E3AD),
        waveform: color!(0xC6AAE8),
        black: color!(0x000000),
      },
      Self::HighContrast => Palette {
        middleground: color!(0x000000),
        foreground: color!(0x111111),
        background: color!(0x000000),
        border: color!(0xcccccc),
        main: color!(0x00ffff),
        text: color!(0xffffff),
        error: color!(0xffff00),
        warning: color!(0xffff00),
        success: color!(0x00ff00),
        waveform: color!(0x00ff00),
        black: color!(0x000000),
      },
      // https://github.com/casperstorm/ferra
      Self::Ferra => Palette {
        middleground: color!(0x2b292d),
        foreground: color!(0x383539),
        background: color!(0x1b1c1e),
        // border: Color::TRANSPARENT,
        border: color!(0x3c393d),
        main: color!(0xfecdb2),
        text: color!(0xd1d1e0),
        error: color!(0xe06b75),
        warning: color!(0xf5d76e),
        success: color!(0xb1b695),
        waveform: color!(0xfecdb2),
        black: color!(0x000000),
      },
    }
  }
}

impl std::fmt::Display for Themes {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Themes::Dark => "Dark",
        Themes::Dracula => "Dracula",
        Themes::Nord => "Nord",
        Themes::Ferra => "Ferra",
        Themes::LMMS => "LMMS",
        Themes::OneShot => "OneShot",
        Themes::Catppuccin => "Catppuccin",
        Themes::HighContrast => "High Contrast",
      }
    )
  }
}
