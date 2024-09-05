// https://github.com/squidowl/halloy/blob/9d0562a4e0a2643daed7283e6737a4307f21b2c6/src/event.rs
// For reference

use iced::{
  self, event,
  keyboard::{self, key::Named},
  window, Subscription,
};

#[derive(Debug, Clone)]
pub enum Event {
  Clear,
  CloseRequested,
  Delete,
  Closed(window::Id),
  Save,
  Load,
}

pub fn events() -> Subscription<Event> {
  iced::event::listen_with(filter)
}

pub fn filter(event: iced::Event, status: event::Status) -> Option<Event> {
  // If the event has not been handled by any widget
  let ignored = |status: event::Status| -> bool { matches!(status, iced::event::Status::Ignored) };

  match event {
    iced::Event::Keyboard(keyboard::Event::KeyReleased { key, modifiers, .. }) => match key.as_ref() {
      keyboard::Key::Named(Named::Delete) if ignored(status) => {
        match modifiers.shift() {
          true => Some(Event::Clear),   // SHIFT + Delete clears the entries
          false => Some(Event::Delete), // Delete will only delete the selected entries
        }
      }
      // CTRL + S or ⌘ + S saves the current configuration
      keyboard::Key::Character("s") if modifiers.command() => Some(Event::Save),
      // CTRL + M or ⌘ + M loads the current configuration
      keyboard::Key::Character("m") if modifiers.command() => Some(Event::Load),
      _ => None,
    },
    iced::Event::Window(id, event) => match event {
      window::Event::CloseRequested => Some(Event::CloseRequested),
      window::Event::Closed => Some(Event::Closed(id)),
      _ => None,
    },
    _ => None,
  }
}
