use iced::widget::container;

use crate::ui::components::Theme;
/// The style of a text input.
#[derive(Default, Clone)]
pub enum Table {
  /// The default style.
  #[default]
  Default,
}

impl crate::ui::components::table::StyleSheet for Theme {
  type Style = Table;

  fn header(&self, _style: &Self::Style) -> container::Appearance {
    let p = self.inner();
    container::Appearance {
      text_color: Some(p.foreground),
      background: Some(p.text.into()),
      ..Default::default()
    }
  }

  fn footer(&self, style: &Self::Style) -> container::Appearance {
    self.header(style)
  }

  fn row(&self, style: &Self::Style, index: usize) -> container::Appearance {
    let p = self.inner();
    let pair = match style {
      Table::Default => {
        if index % 2 == 0 {
          p.foreground
        } else {
          p.middleground
        }
      }
    };
    container::Appearance {
      text_color: Some(p.text),
      background: Some(pair.into()),
      ..Default::default()
    }
  }

  fn divider(&self, style: &Self::Style, _hovered: bool) -> container::Appearance {
    let p = self.inner();
    match style {
      _ => container::Appearance {
        background: Some(p.main.into()),
        text_color: Some(p.foreground.into()),
        ..Default::default()
      },
    }
  }
}
